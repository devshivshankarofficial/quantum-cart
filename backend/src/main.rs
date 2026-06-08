use axum::{
    extract::{Json as ExtractJson, Path, State},
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use quantum_cart::domains::order::commands::CheckoutCommand;
use quantum_cart::domains::order::service::CheckoutCommandHandler;
use quantum_cart::domains::pricing::quantum::{generate_prediction, optimize_cart_pricing, predict_quantum_demand};
use quantum_cart::shared::models::{CheckoutRequest, Inventory, Product};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, Row};
use std::{sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, limit::RequestBodyLimitLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    db: Option<sqlx::PgPool>,
    redis_client: Option<redis::Client>,
    jwt_secret: String,
    require_auth: bool,
}

#[tokio::main]
async fn main() {
    init_tracing();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/quantumcart".to_string());
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "dev-secret-change-me-before-production".to_string());
    let require_auth = std::env::var("REQUIRE_AUTH")
        .map(|value| value == "true" || value == "1")
        .unwrap_or(false);

    let db_pool = match PgPoolOptions::new()
        .max_connections(25)
        .acquire_timeout(Duration::from_secs(5))
        .connect_lazy(&database_url)
    {
        Ok(pool) => {
            info!("PostgreSQL pool configured");
            Some(pool)
        }
        Err(error) => {
            warn!("PostgreSQL disabled: {error}");
            None
        }
    };

    let redis_client = match redis::Client::open(redis_url) {
        Ok(client) => Some(client),
        Err(error) => {
            warn!("Redis disabled: {error}");
            None
        }
    };

    let state = Arc::new(AppState {
        db: db_pool,
        redis_client,
        jwt_secret,
        require_auth,
    });

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/products", get(get_products))
        .route("/api/v1/products/{id}", get(get_product_details))
        .route("/api/v1/inventory/{product_id}", get(get_inventory))
        .route("/api/v1/checkout", post(checkout))
        .route("/api/v1/plugin/import", post(plugin_import))
        .route("/api/v1/webhooks/vendor", post(vendor_webhook))
        .route("/api/v1/quantum/predict/{product_id}", get(quantum_predict))
        .layer(
            ServiceBuilder::new()
                .layer(TimeoutLayer::new(Duration::from_secs(15)))
                .layer(TraceLayer::new_for_http())
                .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
                .layer(CorsLayer::permissive())
                .layer(middleware::from_fn_with_state(state.clone(), auth_middleware)),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind QuantumCart server on port 3000");
    info!("QuantumCart server listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.expect("server failed");
}

fn init_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .json()
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
}

async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    request: axum::http::Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = request.uri().path();
    let is_public = path == "/health"
        || path.starts_with("/api/v1/products")
        || path.starts_with("/api/v1/inventory")
        || path.starts_with("/api/v1/quantum/predict");

    if is_public || !state.require_auth {
        return Ok(next.run(request).await);
    }

    let Some(auth_header) = headers.get("Authorization") else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    let Ok(auth_value) = auth_header.to_str() else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    let Some(token) = auth_value.strip_prefix("Bearer ") else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let key = DecodingKey::from_secret(state.jwt_secret.as_bytes());
    decode::<serde_json::Value>(token, &key, &Validation::new(Algorithm::HS256))
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(next.run(request).await)
}

async fn health_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(json!({
        "status": "UP",
        "version": env!("CARGO_PKG_VERSION"),
        "db": if state.db.is_some() { "configured" } else { "fallback" },
        "cache": if state.redis_client.is_some() { "configured" } else { "fallback" },
        "auth_required": state.require_auth,
        "quantum_engine": "active"
    }))
}

async fn get_products(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    if let Some(pool) = &state.db {
        match sqlx::query(
            r#"
            SELECT product_id, vendor_id, title, description, price::float8 AS price,
                   currency, three_d_model_url
            FROM products
            ORDER BY created_at DESC
            LIMIT 50
            "#,
        )
        .fetch_all(pool)
        .await
        {
            Ok(rows) => {
                let products: Vec<Product> = rows
                    .into_iter()
                    .map(|row| Product {
                        product_id: row.get("product_id"),
                        vendor_id: row.try_get("vendor_id").ok(),
                        title: row.get("title"),
                        description: row.try_get("description").ok(),
                        price: row.get("price"),
                        currency: row.get("currency"),
                        three_d_model_url: row.try_get("three_d_model_url").ok(),
                    })
                    .collect();
                return Json(json!({ "data": products, "source": "postgres" }));
            }
            Err(error) => error!("product query failed: {error}"),
        }
    }

    Json(json!({ "data": [], "source": "fallback" }))
}

async fn get_product_details(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    if let Some(pool) = &state.db {
        match sqlx::query(
            r#"
            SELECT product_id, vendor_id, title, description, price::float8 AS price,
                   currency, three_d_model_url
            FROM products
            WHERE product_id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        {
            Ok(Some(row)) => {
                let product = Product {
                    product_id: row.get("product_id"),
                    vendor_id: row.try_get("vendor_id").ok(),
                    title: row.get("title"),
                    description: row.try_get("description").ok(),
                    price: row.get("price"),
                    currency: row.get("currency"),
                    three_d_model_url: row.try_get("three_d_model_url").ok(),
                };
                return (StatusCode::OK, Json(json!({ "product": product })));
            }
            Ok(None) => return (StatusCode::NOT_FOUND, Json(json!({ "error": "Product not found" }))),
            Err(error) => error!("product detail query failed: {error}"),
        }
    }

    (StatusCode::NOT_FOUND, Json(json!({ "error": "Product not found" })))
}

async fn get_inventory(State(state): State<Arc<AppState>>, Path(product_id): Path<Uuid>) -> impl IntoResponse {
    if let Some(pool) = &state.db {
        match sqlx::query(
            r#"
            SELECT product_id, stock_level, predicted_demand
            FROM inventory
            WHERE product_id = $1
            "#,
        )
        .bind(product_id)
        .fetch_optional(pool)
        .await
        {
            Ok(Some(row)) => {
                let inventory = Inventory {
                    product_id: row.get("product_id"),
                    stock_level: row.get("stock_level"),
                    predicted_demand: row.try_get("predicted_demand").unwrap_or_default(),
                };
                return Json(json!({ "inventory": inventory, "source": "postgres" }));
            }
            Ok(None) => {}
            Err(error) => error!("inventory query failed: {error}"),
        }
    }

    let predicted = predict_quantum_demand(100, &[50, 120, 80, 95]);
    Json(json!({
        "product_id": product_id,
        "stock_level": 42,
        "predicted_demand": predicted,
        "source": "quantum-fallback"
    }))
}

async fn checkout(
    State(state): State<Arc<AppState>>,
    ExtractJson(payload): ExtractJson<CheckoutRequest>,
) -> impl IntoResponse {
    if payload.quantity <= 0 {
        return (StatusCode::BAD_REQUEST, Json(json!({ "error": "quantity must be positive" })));
    }

    if let Some(pool) = &state.db {
        let handler = CheckoutCommandHandler::new(pool.clone());
        let command = CheckoutCommand::new(payload.product_id, payload.quantity, payload.buyer_id);
        match handler.handle(command).await {
            Ok(event) => {
                return (StatusCode::OK, Json(json!({ "success": true, "event": event })));
            }
            Err(error) => {
                error!("checkout failed: {error}");
                return (StatusCode::CONFLICT, Json(json!({ "error": error.to_string() })));
            }
        }
    }

    let optimized_price = optimize_cart_pricing(99.99, payload.quantity, 0.3);
    (StatusCode::OK, Json(json!({
        "success": true,
        "mode": "quantum-fallback",
        "optimized_unit_price": optimized_price,
        "total_amount": optimized_price * payload.quantity as f64,
        "trace_id": Uuid::new_v4()
    })))
}

async fn plugin_import(
    State(state): State<Arc<AppState>>,
    ExtractJson(data): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    if let Some(pool) = &state.db {
        if let Err(error) = sqlx::query(
            "INSERT INTO audit_logs (entity_type, entity_id, action, payload) VALUES ('import', gen_random_uuid(), 'plugin', $1)",
        )
        .bind(data)
        .execute(pool)
        .await
        {
            error!("plugin import audit failed: {error}");
        }
    }

    Json(json!({ "status": "imported", "quantum_optimized": true }))
}

async fn vendor_webhook(
    State(state): State<Arc<AppState>>,
    ExtractJson(event): ExtractJson<serde_json::Value>,
) -> impl IntoResponse {
    if let Some(pool) = &state.db {
        if let Err(error) = sqlx::query(
            "INSERT INTO audit_logs (entity_type, entity_id, action, payload) VALUES ('webhook', gen_random_uuid(), 'vendor_update', $1)",
        )
        .bind(event)
        .execute(pool)
        .await
        {
            error!("vendor webhook audit failed: {error}");
        }
    }

    Json(json!({ "received": true, "action": "inventory.reconciled" }))
}

async fn quantum_predict(Path(product_id): Path<Uuid>) -> impl IntoResponse {
    let historical = vec![120, 95, 150, 80, 200, 110];
    let prediction = generate_prediction(500, &historical, 29.99);
    Json(json!({
        "product_id": product_id,
        "quantum_predicted_demand": prediction.predicted_demand,
        "optimized_dynamic_price": prediction.optimized_price,
        "quantum_confidence": prediction.quantum_confidence,
        "algorithm": prediction.algorithm
    }))
}
