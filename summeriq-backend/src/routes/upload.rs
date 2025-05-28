use axum::{
    extract::{Multipart, State},
    routing::{post, get},
    body::Body,
    response::IntoResponse,
    Json,
    Router,
};
use headers::{Authorization, authorization::Bearer};
use std::sync::Arc;
use serde_json::json;
use tracing::{info, error, debug};
use tokio::fs;
use std::path::PathBuf;
use uuid::Uuid;

use crate::error::AppError;
use crate::auth_middleware::auth_middleware;
use crate::services::{AuthService, StorageService};
use crate::services::auth::AuthUser;
// AppState is assumed to be imported or defined elsewhere

pub fn upload_router(
    auth_service: Arc<AuthService>,
    storage_service: Arc<StorageService>,
    ai_service: Arc<crate::services::AIService>,
) -> Router<(Arc<AuthService>, Arc<StorageService>, Arc<crate::services::AIService>)> {
    Router::new()
        .route("/upload", post(upload_file))
        .route("/debug/buckets", get(list_buckets))
        .layer(axum::middleware::from_fn_with_state(
            (auth_service, storage_service, ai_service),
            auth_middleware,
        ))
}

async fn list_buckets(
    State((_, storage_service, _)): State<(Arc<AuthService>, Arc<StorageService>, Arc<crate::services::AIService>)>,
) -> Json<Vec<String>> {
    match storage_service.list_buckets().await {
        Ok(buckets) => Json(buckets),
        Err(e) => {
            error!("Failed to list buckets: {:?}", e);
            Json(Vec::new())
        }
    }
}

pub async fn upload_file(
    State((_, storage_service, _)): State<(Arc<AuthService>, Arc<StorageService>, Arc<crate::services::AIService>)>,
    auth_user: AuthUser,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    println!("DEBUG: ===== Starting upload_file handler =====");
    println!("DEBUG: Auth user: {:?}", auth_user);
    println!("DEBUG: Storage service bucket: {}", storage_service.bucket_name());
    
    println!("DEBUG: Starting file upload for user: {}", auth_user.0);
    info!("Starting file upload for user: {}", auth_user.0);
    debug!("Received multipart request");

    let mut file_data = None;
    let mut file_name = None;
    let mut content_type = None;

    // Process multipart form data
    println!("DEBUG: Starting to process multipart form data");
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        println!("DEBUG: Failed to read multipart field: {:?}", e);
        error!("Failed to read multipart field: {:?}", e);
        AppError::BadRequest(format!("Failed to read multipart field: {}", e))
    })? {
        let name = field.name().unwrap_or("unnamed").to_string();
        println!("DEBUG: Processing field: {}", name);
        debug!("Processing field: {}", name);

        if name == "file" {
            file_name = field.file_name().map(|f| f.to_string());
            content_type = field.content_type().map(|ct| ct.to_string());
            println!("DEBUG: File details - Name: {:?}, Content-Type: {:?}", file_name, content_type);
            debug!("File details - Name: {:?}, Content-Type: {:?}", file_name, content_type);

            let data = field.bytes().await.map_err(|e| {
                println!("DEBUG: Failed to read field bytes: {:?}", e);
                error!("Failed to read field bytes: {:?}", e);
                AppError::BadRequest(format!("Failed to read uploaded data: {}", e))
            })?;

            println!("DEBUG: Read {} bytes of file data", data.len());
            info!("Read {} bytes of file data", data.len());
            file_data = Some(data);
        }
    }

    // Validate file data
    let file_data = file_data.ok_or_else(|| {
        println!("DEBUG: No file data found in request");
        error!("No file data found in request");
        AppError::BadRequest("No file data found in request".to_string())
    })?;

    // Validate file name
    let file_name = file_name.ok_or_else(|| {
        println!("DEBUG: No filename provided");
        error!("No filename provided");
        AppError::BadRequest("No filename provided".to_string())
    })?;

    // Validate file extension
    if !file_name.ends_with(".zip") {
        println!("DEBUG: Invalid file type: {}", file_name);
        error!("Invalid file type: {}", file_name);
        return Err(AppError::BadRequest("Only .zip files are supported".to_string()));
    }

    // Create temporary file
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join(&file_name);
    println!("DEBUG: Creating temporary file at: {:?}", temp_file_path);
    info!("Creating temporary file at: {:?}", temp_file_path);

    // Write file data to temporary file
    fs::write(&temp_file_path, &file_data).await.map_err(|e| {
        println!("DEBUG: Failed to write temporary file: {:?}", e);
        error!("Failed to write temporary file: {:?}", e);
        AppError::InternalError(format!("Failed to process uploaded file: {}", e))
    })?;

    // Generate storage key
    let storage_key = format!("{}/{}", auth_user.0, file_name);
    println!("DEBUG: Uploading to MinIO with key: {}", storage_key);
    info!("Uploading to MinIO with key: {}", storage_key);

    // Upload to MinIO
    let upload_result = storage_service.upload_file(&temp_file_path, &storage_key).await;

    // Clean up temporary file regardless of upload result
    if let Err(cleanup_err) = fs::remove_file(&temp_file_path).await {
        println!("DEBUG: Failed to remove temporary file: {:?}", cleanup_err);
        error!("Failed to remove temporary file: {:?}", cleanup_err);
    }

    // Handle upload result
    match upload_result {
        Ok(_) => {
            println!("DEBUG: Successfully uploaded file to MinIO");
            info!("Successfully uploaded file to MinIO");
            
            Ok(Json(json!({
                "message": "File uploaded successfully",
                "fileName": file_name,
                "key": storage_key,
                "contentType": content_type.unwrap_or_else(|| "application/zip".to_string())
            })))
        }
        Err(e) => {
            println!("DEBUG: Failed to upload to MinIO: {:?}", e);
            error!("Failed to upload to MinIO: {:?}", e);
            Err(AppError::StorageError(format!("Failed to upload file: {}", e)))
        }
    }
}
