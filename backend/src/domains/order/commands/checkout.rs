//! Checkout Command for CQRS (Command Query Responsibility Segregation)
//! Represents a request to purchase a product.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The command to initiate a checkout process.
#[derive(Debug, Deserialize, Serialize)]
pub struct CheckoutCommand {
    pub product_id: Uuid,
    pub quantity: i32,
    pub buyer_id: Option<Uuid>,
}

impl CheckoutCommand {
    pub fn new(product_id: Uuid, quantity: i32, buyer_id: Option<Uuid>) -> Self {
        Self {
            product_id,
            quantity,
            buyer_id,
        }
    }
}