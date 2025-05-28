use axum::{
    extract::{Multipart, State},
    Json,
};
use std::sync::Arc;
use tracing::{info, error, debug};
use crate::{
    auth::Auth,
    models::upload::UploadResponse,
    services::{StorageService, UploadService},
    AppState,
    AppError,
};

pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    auth: Auth,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    info!("Starting file upload for user: {}", auth.user_id);
    
    let mut file_data = Vec::new();
    let mut file_name = String::new();
    let mut content_type = String::new();

    debug!("Processing multipart form data");
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        error!("Error reading multipart field: {}", e);
        AppError::ValidationError(format!("Error reading multipart field: {}", e))
    })? {
        let name = field.name().unwrap_or("unnamed").to_string();
        debug!("Processing multipart field: {}", name);

        if name == "file" {
            file_name = field.file_name()
                .unwrap_or("unnamed")
                .to_string();
            content_type = field.content_type()
                .unwrap_or("application/octet-stream")
                .to_string();
            
            debug!("File details - Name: {}, Content-Type: {}", file_name, content_type);
            
            file_data = field.bytes().await.map_err(|e| {
                error!("Error reading file bytes: {}", e);
                AppError::ValidationError(format!("Error reading file bytes: {}", e))
            })?;
            
            debug!("Successfully read file data, size: {} bytes", file_data.len());
        }
    }

    if file_data.is_empty() {
        error!("No file data received in request");
        return Err(AppError::ValidationError("No file data received".to_string()));
    }

    debug!("Creating upload record in database");
    let upload = state.upload_service.create_upload(
        auth.user_id,
        &file_name,
        &content_type,
        file_data.len() as i64,
    ).await.map_err(|e| {
        error!("Database error creating upload: {}", e);
        AppError::DatabaseError(e)
    })?;

    debug!("Upload record created with ID: {}", upload.id);
    debug!("Attempting to store file in S3");
    
    let bucket_name = "uploaded-folder";
    let key = format!("uploads/{}", upload.id);
    
    debug!("S3 storage details - Bucket: {}, Key: {}", bucket_name, key);

    state.storage_service.store_file(
        bucket_name,
        &key,
        file_data,
    ).await.map_err(|e| {
        error!("Storage error: {}", e);
        AppError::StorageError(format!("Failed to store file: {}", e))
    })?;

    info!("File successfully stored in S3");
    info!("Upload completed successfully for user: {}", auth.user_id);

    Ok(Json(UploadResponse {
        upload_id: upload.id,
        file_name,
        content_type,
        size: file_data.len() as i64,
    }))
} 