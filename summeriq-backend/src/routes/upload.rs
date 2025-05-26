use axum::{
    extract::{Multipart, State},
    routing::{post, get},
    Router,
    Json,
    middleware,
};
use std::sync::Arc;
use crate::services::{AuthService, StorageService};
use crate::services::auth::AuthUser;
use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;
use serde_json::json;
use crate::error::AppError;
use crate::auth_middleware::auth_middleware;

pub fn upload_router(
    auth_service: Arc<AuthService>,
    storage_service: Arc<StorageService>,
    ai_service: Arc<crate::services::AIService>,
) -> Router<(Arc<AuthService>, Arc<StorageService>, Arc<crate::services::AIService>)> {
    tracing::info!("Initializing upload router");
    let router = Router::new()
        .route("/upload", post(upload_file))
        .route("/debug/buckets", get(list_buckets))
        .layer(middleware::from_fn_with_state(
            (auth_service, storage_service, ai_service),
            auth_middleware,
        ));
    tracing::info!("Upload router initialized with routes: /upload (POST), /debug/buckets (GET)");
    router
}

async fn list_buckets(
    State((_, storage_service, _)): State<(Arc<AuthService>, Arc<StorageService>, Arc<crate::services::AIService>)>,
) -> Json<Vec<String>> {
    match storage_service.list_buckets().await {
        Ok(buckets) => Json(buckets),
        Err(e) => {
            tracing::error!("Failed to list buckets: {}", e);
            Json(Vec::new())
        }
    }
}

async fn upload_file(
    State((_, storage_service, _)): State<(Arc<AuthService>, Arc<StorageService>, Arc<crate::services::AIService>)>,
    auth_user: AuthUser,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, AppError> {
    tracing::info!("Starting file upload for user: {}", auth_user.0);
    tracing::debug!("Received multipart request");
    
    // Get the first field from the multipart form
    let field = match multipart.next_field().await {
        Ok(Some(field)) => {
            tracing::info!("Successfully got multipart field");
            field
        },
        Ok(None) => {
            tracing::error!("No multipart field found in request");
            return Err(AppError::ValidationError("No file provided".to_string()));
        }
        Err(e) => {
            tracing::error!("Failed to get multipart field: {}", e);
            return Err(AppError::ValidationError(format!("Failed to process uploaded file: {}", e)));
        }
    };

    // Get the field name to ensure it's 'file'
    let field_name = field.name().unwrap_or_default();
    tracing::info!("Processing field: {}", field_name);
    
    if field_name != "file" {
        tracing::error!("Invalid field name: {}", field_name);
        return Err(AppError::ValidationError("Invalid field name. Expected 'file'".to_string()));
    }

    let file_name = match field.file_name() {
        Some(name) => {
            tracing::info!("File name from request: {}", name);
            name.to_string()
        },
        None => {
            tracing::error!("No filename provided in upload");
            return Err(AppError::ValidationError("No filename provided".to_string()));
        }
    };
    
    tracing::info!("Processing file: {}", file_name);
    
    // Read the file data
    let data = match field.bytes().await {
        Ok(data) => {
            tracing::info!("Successfully read file data, size: {} bytes", data.len());
            data
        },
        Err(e) => {
            tracing::error!("Failed to read file data: {}", e);
            return Err(AppError::FileError(format!("Failed to read file data: {}", e)));
        }
    };
    
    // Create a temporary file
    let temp_path = PathBuf::from(format!("/tmp/{}", Uuid::new_v4()));
    tracing::info!("Creating temporary file at: {:?}", temp_path);
    
    if let Err(e) = fs::write(&temp_path, &data).await {
        tracing::error!("Failed to write temporary file: {}", e);
        return Err(AppError::FileError(format!("Failed to process file: {}", e)));
    }
    
    tracing::info!("Temporary file written successfully");
    
    // Generate the storage key
    let key = format!("{}/{}", auth_user.0, file_name);
    tracing::info!("Uploading to MinIO with key: {}", key);
    
    // Upload to MinIO
    if let Err(e) = storage_service.upload_file(&temp_path, &key).await {
        tracing::error!("Failed to upload file to storage: {}", e);
        // Clean up temp file before returning error
        if let Err(cleanup_err) = fs::remove_file(&temp_path).await {
            tracing::warn!("Failed to remove temporary file: {}", cleanup_err);
        }
        return Err(AppError::StorageError(format!("Failed to upload file: {}", e)));
    }
    
    tracing::info!("File uploaded successfully to MinIO");
    
    // Clean up temp file
    if let Err(e) = fs::remove_file(&temp_path).await {
        tracing::warn!("Failed to remove temporary file: {}", e);
    }
    
    Ok(Json(json!({
        "key": key,
        "fileName": file_name,
        "success": true
    })))
}
