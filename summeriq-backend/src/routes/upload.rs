use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use serde_json::json;
use tracing::{info, error};
use uuid::Uuid;

use crate::error::AppError;
use crate::services::StorageService;

pub async fn upload_file(
    storage_service: web::Data<StorageService>,
    mut payload: Multipart,
) -> Result<impl Responder, AppError> {
    let mut file_content = Vec::new();
    let mut filename = None;

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        filename = content_disposition.get_filename().map(|s| s.to_string());

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file_content.extend_from_slice(&data);
        }
    }

    let filename = filename.ok_or_else(|| AppError::UploadError("No filename provided".to_string()))?;
    let file_id = Uuid::new_v4();
    let final_filename = format!("{}_{}", file_id, filename);

    storage_service.save_file(&file_content, &final_filename).await?;

    info!("File uploaded successfully: {}", final_filename);
    Ok(HttpResponse::Created().json(json!({
        "message": "File uploaded successfully",
        "file_id": file_id,
        "filename": final_filename
    })))
}

pub async fn get_file(
    storage_service: web::Data<StorageService>,
    file_id: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let content = storage_service.read_file(&file_id).await?;
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(content))
} 