use axum::{
    extract::{Multipart, State},
    routing::post,
    response::IntoResponse,
    Json,
    Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::io::Write;
use serde_json::json;
use tracing::{info, error, debug, warn};
use tokio::fs;
use chrono::Utc;
use tempfile::NamedTempFile;

use crate::error::AppError;
use crate::auth_middleware::auth_middleware;
use crate::services::{AuthService, StorageService};
use crate::services::auth::AuthUser;
use crate::models::upload::Upload;

type AppState = (Arc<AuthService>, Arc<StorageService>, Arc<crate::services::AIService>);

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    message: String,
    filename: String,
}

pub fn upload_router(
    auth_service: Arc<AuthService>,
    storage_service: Arc<StorageService>,
    ai_service: Arc<crate::services::AIService>,
) -> Router<AppState> {
    println!("DEBUG: Creating upload router");
    Router::new()
        .route("/upload", post(upload_file))
        .route("/upload/test", post(test_upload))
        .layer(axum::middleware::from_fn_with_state(
            (auth_service, storage_service, ai_service),
            auth_middleware,
        ))
}

// Test endpoint that just returns OK
async fn test_upload(
    State((_auth_service, _storage_service, _ai_service)): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<serde_json::Value>, AppError> {
    println!("DEBUG: Test upload endpoint reached");
    println!("DEBUG: User: {}", auth_user.0);
    Ok(Json(json!({
        "status": "ok",
        "message": "Test endpoint working",
        "user_id": auth_user.0
    })))
}

pub async fn upload_file(
    State((_auth_service, storage_service, _ai_service)): State<AppState>,
    auth_user: AuthUser,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    println!("DEBUG: ===== Starting file upload =====");
    println!("DEBUG: User ID: {:?}", auth_user.0);

    // Process multipart fields
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("unknown").to_string();
        println!("DEBUG: Processing field: {}", name);
        
        let file_name = field.file_name().map(|s| s.to_string());
        println!("DEBUG: Field file name: {:?}", file_name);

        if let Some(name) = file_name {
            println!("DEBUG: Found file: {}", name);
            match field.bytes().await {
                Ok(data) => {
                    println!("DEBUG: Read {} bytes", data.len());
                    // Upload the file
                    match storage_service.upload_file_bytes(data.to_vec(), &name).await {
                        Ok(file_id) => {
                            println!("DEBUG: File uploaded successfully with ID: {}", file_id);
                            return Ok(Json(json!({
                                "message": "File uploaded successfully",
                                "file_id": file_id
                            })));
                        }
                        Err(e) => {
                            println!("DEBUG: Failed to upload file: {:?}", e);
                            return Err(AppError::UploadError(format!("Failed to upload file: {}", e)));
                        }
                    }
                }
                Err(e) => {
                    println!("DEBUG: Error reading file data: {:?}", e);
                    return Err(AppError::UploadError(format!("Failed to read file data: {}", e)));
                }
            }
        } else {
            println!("DEBUG: Field has no file name, skipping");
        }
    }

    println!("DEBUG: No file found in request");
    Err(AppError::UploadError("No file found in request".to_string()))
}
