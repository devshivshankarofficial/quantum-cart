//! Events for the Order Context (CQRS Event Sourcing)

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Event emitted when a transaction is completed.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionCompletedEvent {
    pub event_id: Uuid,
    pub transaction_id: Uuid,
    pub product_id: Uuid,
    pub vendor_id: Uuid,
    pub quantity: i32,
    pub total_amount: f64,
    pub status: String,
    pub occurred_at: DateTime<Utc>,
    pub trace_id: String,
}

impl TransactionCompletedEvent {
    pub fn new(
        transaction_id: Uuid,
        product_id: Uuid,
        vendor_id: Uuid,
        quantity: i32,
        total_amount: f64,
        status: String,
        trace_id: String,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            transaction_id,
            product_id,
            vendor_id,
            quantity,
            total_amount,
            status,
            occurred_at: Utc::now(),
            trace_id,
        }
    }
}