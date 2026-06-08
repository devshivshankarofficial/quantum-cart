//! Order Service - Contains domain logic for order processing, including CQRS command handlers.

use crate::domains::order::commands::CheckoutCommand;
use crate::domains::order::events::TransactionCompletedEvent;
use crate::domains::pricing::quantum::optimize_cart_pricing;
use crate::shared::errors::AppError;
use sqlx::PgPool;
use sqlx::Row;
use tracing::error;
use uuid::Uuid;

/// Handles the CheckoutCommand, performing the transactional checkout process.
pub struct CheckoutCommandHandler {
    pub pool: PgPool,
}

impl CheckoutCommandHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Process the checkout command.
    /// Returns a TransactionCompletedEvent on success, or an AppError on failure.
    pub async fn handle(&self, command: CheckoutCommand) -> Result<TransactionCompletedEvent, AppError> {
        let CheckoutCommand {
            product_id,
            quantity,
            buyer_id,
        } = command;

        // Start a transaction: check stock, deduct, insert transaction record.
        let result = sqlx::query(
            r#"
            WITH stock_check AS (
                SELECT stock_level FROM inventory WHERE product_id = $1 FOR UPDATE
            ),
            update_inv AS (
                UPDATE inventory SET stock_level = stock_level - $2 
                WHERE product_id = $1 AND stock_level >= $2
                RETURNING stock_level
            ),
            new_tx AS (
                INSERT INTO transactions (transaction_id, product_id, buyer_id, vendor_id, quantity, total_amount, status, created_at)
                SELECT gen_random_uuid(), $1, $3, (SELECT vendor_id FROM products WHERE product_id=$1 LIMIT 1), $2, 
                       (SELECT price * $2 FROM products WHERE product_id = $1), 'COMPLETED', now()
                RETURNING transaction_id, total_amount
            )
            SELECT n.transaction_id, n.total_amount::float8 AS total_amount FROM new_tx n, update_inv
            "#,
        )
        .bind(product_id)
        .bind(quantity)
        .bind(buyer_id)
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(row) => {
                let transaction_id = row.try_get::<Uuid, _>("transaction_id").unwrap_or_else(|_| Uuid::new_v4());
                let total_amount = row.try_get::<f64, _>("total_amount").unwrap_or(0.0);
                // In a real system, we would get the vendor_id from the product, but for simplicity we'll use a placeholder.
                let vendor_id = Uuid::new_v4(); // TODO: fetch from product

                // Create the completed event.
                let event = TransactionCompletedEvent::new(
                    transaction_id,
                    product_id,
                    vendor_id,
                    quantity,
                    total_amount,
                    "COMPLETED".to_string(),
                    Uuid::new_v4().to_string(), // trace_id
                );

                // Persist the event to the event_store (or audit_logs for now).
                // We'll use the audit_logs table as a placeholder until we have a full event store implementation.
                sqlx::query(
                    r#"
                    INSERT INTO audit_logs (entity_type, entity_id, action, payload)
                    VALUES ('transaction', $1, 'completed', $2)
                    "#,
                )
                .bind(transaction_id)
                .bind(serde_json::json!({
                        "event_id": event.event_id,
                        "transaction_id": event.transaction_id,
                        "product_id": event.product_id,
                        "quantity": event.quantity,
                        "total_amount": event.total_amount,
                        "trace_id": event.trace_id
                    }))
                .execute(&self.pool)
                .await?;

                Ok(event)
            }
            Err(e) => {
                error!("Checkout transaction failed (stock or db): {}", e);
                Err(AppError::Database(e))
            }
        }
    }

    // Fallback quantum optimized path (if DB is not available, we can use quantum pricing for simulation)
    #[allow(dead_code)]
    pub async fn handle_fallback(&self, command: CheckoutCommand) -> Result<TransactionCompletedEvent, AppError> {
        let CheckoutCommand {
            product_id,
            quantity,
            buyer_id: _,
        } = command;

        // Simulate a quantum optimized price.
        let base_price = 99.99; // This would come from the product in reality.
        let optimized_price = optimize_cart_pricing(base_price, quantity, 0.3);
        let total_amount = optimized_price * quantity as f64;

        let transaction_id = Uuid::new_v4();
        let vendor_id = Uuid::new_v4(); // placeholder

        let event = TransactionCompletedEvent::new(
            transaction_id,
            product_id,
            vendor_id,
            quantity,
            total_amount,
            "COMPLETED".to_string(),
            Uuid::new_v4().to_string(),
        );

        // Persist the fallback event as well.
        sqlx::query(
            r#"
            INSERT INTO audit_logs (entity_type, entity_id, action, payload)
            VALUES ('transaction', $1, 'completed_fallback', $2)
            "#,
        )
        .bind(transaction_id)
        .bind(serde_json::json!({
                "event_id": event.event_id,
                "transaction_id": event.transaction_id,
                "product_id": event.product_id,
                "quantity": event.quantity,
                "total_amount": event.total_amount,
                "trace_id": event.trace_id,
                "mode": "quantum-fallback"
            }))
        .execute(&self.pool)
        .await?;

        Ok(event)
    }
}