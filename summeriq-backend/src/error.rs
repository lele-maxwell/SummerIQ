use actix_web::{HttpResponse, ResponseError};
use actix_multipart::MultipartError;
use sqlx::Error as SqlxError;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),
    
    #[error("IO error: {0}")]
    IoError(#[from] IoError),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Upload error: {0}")]
    UploadError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("File error: {0}")]
    FileError(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl From<MultipartError> for AppError {
    fn from(error: MultipartError) -> Self {
        AppError::UploadError(error.to_string())
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        tracing::error!("Returning error: {:?}", self);
        
        match self {
            AppError::AuthenticationError(_) => {
                HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": self.to_string()
                }))
            }
            AppError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Database error occurred"
                }))
            }
            AppError::IoError(_) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "IO error occurred"
                }))
            }
            AppError::StorageError(_) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": self.to_string()
                }))
            }
            AppError::UploadError(_) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": self.to_string()
                }))
            }
            AppError::ValidationError(_) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": self.to_string()
                }))
            }
            AppError::FileError(_) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": self.to_string()
                }))
            }
            AppError::BadRequest(_) => {
                HttpResponse::BadRequest().json(serde_json::json!({
                    "error": self.to_string()
                }))
            }
            AppError::InternalServerError(_) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Internal server error occurred"
                }))
            }
        }
    }
}
