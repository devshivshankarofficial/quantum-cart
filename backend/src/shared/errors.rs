use thiserror::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database operation failed: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Insufficient stock for product {0}")]
    OutOfStock(String),
    
    #[error("Quantum prediction failed: {0}")]
    QuantumPredictionFailed(String),
    
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::OutOfStock(_) => (StatusCode::CONFLICT, self.to_string()),
            AppError::AuthFailed(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".to_string()),
        };
        
        let body = Json(json!({
            "error": error_message,
            "code": status.as_u16()
        }));
        
        (status, body).into_response()
    }
}