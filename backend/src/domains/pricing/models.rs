use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceRequest {
    pub product_id: Uuid,
    pub base_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceResponse {
    pub product_id: Uuid,
    pub optimized_price: f64,
    pub quantum_demand: i32,
    pub confidence: f64,
}