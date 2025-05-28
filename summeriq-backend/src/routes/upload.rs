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
    State((_storage_service, _)): State<(Arc<AuthService>, Arc<StorageService>, Arc<crate::services::AIService>)>,
    auth_user: AuthUser,
    // mut multipart: Multipart, // Temporarily commented out for diagnostics
) -> Result<impl IntoResponse, AppError> {
    tracing::error!("!!!!!!!!!!!!!!!!!!!! ENTERING UPLOAD_FILE HANDLER !!!!!!!!!!!!!!!!!!!!");
    debug!("Auth user: {:?}", auth_user); // Keep this for basic check if auth middleware runs

    // Temporarily skip all multipart processing for diagnostics
    Ok(Json(json!({"message": "Handler entered for diagnostics. Multipart processing skipped."})))

    /*
    debug!("===== Starting upload_file handler =====");
    debug!("Storage service bucket: {}", _storage_service.bucket_name());
    
    info!("Starting file upload for user: {}", auth_user.0);
    debug!("Received multipart request");

    let mut file_data = None;
    let mut file_name = None;
    let mut content_type = None;

    // Process multipart form data
    debug!("Starting to process multipart form data");
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        let field_name = field.name().unwrap_or("unnamed").to_string();
        error!("Failed to read multipart field '{}': {:?}", field_name, e);
        AppError::BadRequest(format!("Failed to read multipart field '{}': {}", field_name, e))
    })? {
        let name = field.name().unwrap_or("unnamed").to_string();
        debug!("Processing field: {}", name);

        if name == "file" {
            file_name = field.file_name().map(|f| f.to_string());
            content_type = field.content_type().map(|ct| ct.to_string());
            debug!("File details - Name: {:?}, Content-Type: {:?}", file_name, content_type);

            let data = field.bytes().await.map_err(|e| {
                error!("Failed to read field bytes for field '{}': {:?}", name, e);
                AppError::BadRequest(format!("Failed to read uploaded data for field '{}': {}", name, e))
            })?;

            info!("Read {} bytes of file data for field '{}'", data.len(), name);
            file_data = Some(data);
        }
    }

    // Validate file data
    let file_data = file_data.ok_or_else(|| {
        error!("No file data found in request for user {}", auth_user.0);
        AppError::BadRequest("No file data found in request".to_string())
    })?;

    // Validate file name
    let file_name = file_name.ok_or_else(|| {
        error!("No filename provided in request for user {}", auth_user.0);
        AppError::BadRequest("No filename provided".to_string())
    })?;

    // Validate file extension
    if !file_name.ends_with(".zip") {
        error!("Invalid file type: {} for user {}", file_name, auth_user.0);
        return Err(AppError::BadRequest("Only .zip files are supported".to_string()));
    }

    // Create temporary file
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join(&file_name);
    info!("Creating temporary file at: {:?} for user {}", temp_file_path, auth_user.0);

    // Write file data to temporary file
    fs::write(&temp_file_path, &file_data).await.map_err(|e| {
        error!("Failed to write temporary file '{:?}': {:?}", temp_file_path, e);
        AppError::InternalError(format!("Failed to process uploaded file: {}", e))
    })?;

    // Generate storage key
    let storage_key = format!("{}/{}", auth_user.0, file_name);
    info!("Uploading to MinIO with key: {} for user {}", storage_key, auth_user.0);

    // Upload to MinIO
    let bucket_name = _storage_service.bucket_name();
    let upload_result = _storage_service.upload_file(&temp_file_path, &storage_key).await;

    // Clean up temporary file regardless of upload result
    if let Err(cleanup_err) = fs::remove_file(&temp_file_path).await {
        error!("Failed to remove temporary file '{:?}': {:?}", temp_file_path, cleanup_err);
    }

    // Handle upload result
    match upload_result {
        Ok(_) => {
            info!("Successfully uploaded file '{}' to MinIO bucket '{}' with key '{}' for user {}", file_name, bucket_name, storage_key, auth_user.0);
            
            Ok(Json(json!({
                "message": "File uploaded successfully",
                "fileName": file_name.clone(),
                "key": storage_key.clone(),
                "contentType": content_type.unwrap_or_else(|| "application/zip".to_string())
            })))
        }
        Err(e) => {
            error!("Failed to upload file '{}' to MinIO bucket '{}' with key '{}': {:?} for user {}", file_name, bucket_name, storage_key, e, auth_user.0);
            Err(AppError::StorageError(format!("Failed to upload file '{}' to bucket '{}' with key '{}': {}", file_name, bucket_name, storage_key, e)))
        }
    }
    */
}
