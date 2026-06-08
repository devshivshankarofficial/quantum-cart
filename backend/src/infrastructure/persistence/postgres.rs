//! PostgreSQL repository implementations (CQRS read/write models)

use crate::shared::errors::AppError;
use crate::shared::models::Product;
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub struct PostgresProductRepo {
    pool: PgPool,
}

impl PostgresProductRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Example repository method for future full CQRS
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT product_id, vendor_id, title, description, price::float8 AS price,
                   currency, three_d_model_url
            FROM products
            WHERE product_id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| Product {
            product_id: row.get("product_id"),
            vendor_id: row.try_get("vendor_id").ok(),
            title: row.get("title"),
            description: row.try_get("description").ok(),
            price: row.get("price"),
            currency: row.get("currency"),
            three_d_model_url: row.try_get("three_d_model_url").ok(),
        }))
    }
}