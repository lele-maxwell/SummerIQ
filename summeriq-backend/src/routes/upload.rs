use axum::{
    extract::{Multipart, State},
    routing::post,
    Router,
    Json,
};
use crate::{
    error::AppError,
    models::upload::{Upload, CreateUpload},
    services::{storage::StorageService, ai::AIService},
};
use sqlx::PgPool;
use uuid::Uuid;
use std::path::PathBuf;
use tempfile::tempdir;
use zip::ZipArchive;
use std::io::Cursor;

pub fn upload_router() -> Router<(PgPool, StorageService, AIService)> {
    Router::new()
        .route("/upload", post(upload))
}

async fn upload(
    State((pool, storage, ai)): State<(PgPool, StorageService, AIService)>,
    mut multipart: Multipart,
) -> Result<Json<Upload>, AppError> {
    let mut file_data = Vec::new();
    let mut file_name = String::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::FileError(e.to_string()))? {
        if field.name() == Some("file") {
            file_name = field.file_name()
                .ok_or_else(|| AppError::FileError("No file name provided".into()))?
                .to_string();
            file_data = field.bytes().await.map_err(|e| AppError::FileError(e.to_string()))?;
        }
    }

    // Create temporary directory for extraction
    let temp_dir = tempdir().map_err(|e| AppError::FileError(e.to_string()))?;
    let temp_path = temp_dir.path();

    // Extract zip file
    let cursor = Cursor::new(file_data);
    let mut archive = ZipArchive::new(cursor)
        .map_err(|e| AppError::FileError(format!("Failed to read zip file: {}", e)))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| AppError::FileError(format!("Failed to read file in zip: {}", e)))?;
        
        let outpath = temp_path.join(file.name());
        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)
                .map_err(|e| AppError::FileError(format!("Failed to create directory: {}", e)))?;
        } else {
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| AppError::FileError(format!("Failed to create parent directory: {}", e)))?;
            }
            let mut outfile = std::fs::File::create(&outpath)
                .map_err(|e| AppError::FileError(format!("Failed to create file: {}", e)))?;
            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| AppError::FileError(format!("Failed to write file: {}", e)))?;
        }
    }

    // Upload to MinIO
    let minio_key = storage.upload_file(&temp_path, Uuid::new_v4()).await?;

    // Create upload record
    let upload = sqlx::query_as!(
        Upload,
        r#"
        INSERT INTO uploads (user_id, file_name, minio_key)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        Uuid::new_v4(), // TODO: Get from auth
        file_name,
        minio_key
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(upload))
}
