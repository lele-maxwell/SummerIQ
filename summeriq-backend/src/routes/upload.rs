use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use serde_json::json;
use tracing::{info, error};
use uuid::Uuid;

use crate::error::AppError;
use crate::services::StorageService;
pub use crate::handlers::upload::upload_file;

pub async fn get_file(
    storage_service: web::Data<StorageService>,
    file_id: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let content = storage_service.read_file(&file_id).await?;
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(content))
}

pub async fn get_file_content(
    storage_service: web::Data<StorageService>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let content = storage_service.read_file(&path).await?;
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(content))
} 