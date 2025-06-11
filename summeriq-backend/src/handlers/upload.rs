use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder, HttpRequest};
use futures::{StreamExt, TryStreamExt};
use serde_json::json;
use tracing::{info, error};
use uuid::Uuid;

use crate::config::Config;
use crate::models::upload::{Upload, CreateUpload};
use crate::services::StorageService;
use crate::services::AuthService;

pub async fn upload_file(
    config: web::Data<Config>,
    storage_service: web::Data<StorageService>,
    auth_service: web::Data<AuthService>,
    req: HttpRequest,
    mut payload: Multipart,
) -> Result<impl Responder, crate::error::AppError> {
    // Get user ID from auth token
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| crate::error::AppError::AuthenticationError("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| crate::error::AppError::AuthenticationError("Invalid token format".to_string()))?;

    let user_id = auth_service.verify_token(token)?;

    let mut file_content = Vec::new();
    let mut filename = None;
    let mut mime_type = None;

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        filename = content_disposition.get_filename().map(|s| s.to_string());
        mime_type = field.content_type().map(|m| m.to_string());

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file_content.extend_from_slice(&data);
        }
    }

    let filename = filename.ok_or_else(|| crate::error::AppError::UploadError("No filename provided".to_string()))?;
    let mime_type = mime_type.unwrap_or_else(|| "application/octet-stream".to_string());
    let file_id = Uuid::new_v4();
    let final_filename = format!("{}_{}", file_id, filename);

    // Save file to storage
    storage_service.save_file(&file_content, &final_filename).await?;

    // Create database record
    let upload = CreateUpload {
        filename: final_filename.clone(),
        mime_type,
        size: file_content.len() as i64,
    };

    let upload = sqlx::query_as!(
        Upload,
        r#"
        INSERT INTO uploads (user_id, filename, original_filename, mime_type, size)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, user_id, filename, original_filename, mime_type, size, created_at, updated_at
        "#,
        user_id,
        upload.filename,
        filename,
        upload.mime_type,
        upload.size
    )
    .fetch_one(&auth_service.pool)
    .await?;

    info!("File uploaded successfully: {}", final_filename);
    Ok(HttpResponse::Created().json(json!({
        "message": "File uploaded successfully",
        "file_id": file_id,
        "filename": final_filename,
        "upload": upload
    })))
}

pub async fn get_file(
    storage_service: web::Data<StorageService>,
    file_id: web::Path<Uuid>,
) -> Result<impl Responder, crate::error::AppError> {
    let content = storage_service.read_file(&file_id.to_string()).await?;
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(content))
} 