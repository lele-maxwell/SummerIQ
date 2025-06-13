use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder, HttpRequest};
use futures::{StreamExt, TryStreamExt};
use serde_json::json;
use tracing::{info, error};
use uuid::Uuid;
use crate::services::storage::FileNode;
use serde_json::Value;
use sqlx::types::Json;
use serde::Serialize;

use crate::config::Config;
use crate::models::upload::{Upload, CreateUpload};
use crate::services::StorageService;
use crate::services::AuthService;

#[derive(Serialize)]
struct UploadResponse {
    message: String,
    file_id: Uuid,
    filename: String,
    upload: UploadRecord,
}

#[derive(Serialize)]
struct UploadRecord {
    id: Uuid,
    user_id: Uuid,
    filename: String,
    original_filename: String,
    mime_type: String,
    size: i64,
    extracted_files: Option<Value>,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn upload_file(
    config: web::Data<Config>,
    storage_service: web::Data<StorageService>,
    auth_service: web::Data<AuthService>,
    req: HttpRequest,
    mut payload: Multipart,
) -> Result<HttpResponse, crate::error::AppError> {
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

    // Extract ZIP file if it's a ZIP or SIP
    let extracted_files = if mime_type == "application/zip" || filename.ends_with(".zip") || filename.ends_with(".sip") {
        let extract_dir = format!("extracted_{}", file_id);
        storage_service.extract_zip(&file_content, &extract_dir).await?;
        let files = storage_service.list_files(&extract_dir).await?;
        Some(serde_json::to_value(files).map_err(|e| crate::error::AppError::InternalServerError(e.to_string()))?)
    } else {
        None
    };

    let extracted_files_json = extracted_files.map(Json);

    let rec = sqlx::query!(
        r#"
        INSERT INTO uploads (user_id, filename, original_filename, mime_type, size, extracted_files)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, user_id, filename, original_filename, mime_type, size, extracted_files as "extracted_files: Json<Value>", created_at, updated_at
        "#,
        user_id,
        final_filename,
        filename,
        mime_type,
        file_content.len() as i64,
        extracted_files_json.map(|v| v.0)
    )
    .fetch_one(&auth_service.pool)
    .await?;

    let upload_record = UploadRecord {
        id: rec.id,
        user_id: rec.user_id,
        filename: rec.filename,
        original_filename: rec.original_filename,
        mime_type: rec.mime_type,
        size: rec.size,
        extracted_files: rec.extracted_files.map(|v| v.0),
        created_at: rec.created_at,
        updated_at: rec.updated_at,
    };

    info!("File uploaded successfully: {}", final_filename);
    Ok(HttpResponse::Created().json(UploadResponse {
        message: "File uploaded successfully".to_string(),
        file_id,
        filename: final_filename,
        upload: upload_record,
    }))
}

pub async fn get_file(
    storage_service: web::Data<StorageService>,
    file_id: web::Path<Uuid>,
) -> Result<HttpResponse, crate::error::AppError> {
    let content = storage_service.read_file(&file_id.to_string()).await?;
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(content))
} 