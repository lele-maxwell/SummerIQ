use actix_web::{web, HttpResponse, Responder};
use tracing::info;

pub async fn health_check() -> impl Responder {
    info!("Health check requested");
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Service is healthy"
    }))
}
