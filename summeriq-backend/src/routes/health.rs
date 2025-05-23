use axum::{
    routing::get,
    Router,
    Json,
};
use serde_json::json;

pub fn health_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
