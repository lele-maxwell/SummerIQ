use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use serde_json::json;
use tracing::{info, error};
use uuid::Uuid;
use urlencoding::decode;

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
    // Decode the URL-encoded path
    let decoded_path = match decode(&path) {
        Ok(path) => path,
        Err(e) => {
            info!("Error decoding path: {}", e);
            return Err(AppError::BadRequest(format!("Invalid path encoding: {}", e)));
        }
    };
    
    // Remove leading slash and get the project name from the path
    let trimmed = decoded_path.trim_start_matches('/');
    let path_parts: Vec<&str> = trimmed.split('/').collect();
    
    if path_parts.is_empty() {
        return Err(AppError::BadRequest("Empty path provided".to_string()));
    }
    
    // Get the project name (first part) and the rest of the path
    let project = path_parts[0];
    let inner_path = path_parts[1..].join("/");
    
    // Get the UUID from the storage service
    let uuid = storage_service.get_file_id(project).await?;
    
    // Construct the full path with storage directory and UUID
    let full_path = if inner_path.is_empty() {
        format!("extracted_{}/{}", uuid, project)
    } else {
        format!("extracted_{}/{}", uuid, inner_path)
    };
    
    info!("Reading file from path: {}", full_path);
    let content = storage_service.read_file(&full_path).await?;
    
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(content))
} 