use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub product_id: Uuid,
    pub vendor_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub price: f64,
    pub currency: String,
    pub three_d_model_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    pub product_id: Uuid,
    pub stock_level: i32,
    pub predicted_demand: i32,
}

#[derive(Debug, Deserialize)]
pub struct CheckoutRequest {
    pub product_id: Uuid,
    pub quantity: i32,
    pub buyer_id: Option<Uuid>,
}