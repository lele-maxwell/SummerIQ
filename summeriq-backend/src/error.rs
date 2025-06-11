use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use sqlx::Error as SqlxError;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),
    
    #[error("IO error: {0}")]
    IoError(#[from] IoError),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Upload error: {0}")]
    UploadError(String),
    
    #[error("AI service error: {0}")]
    AIServiceError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("File error: {0}")]
    FileError(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::AuthError(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::IoError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::StorageError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::UploadError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::AIServiceError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::FileError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        tracing::error!("Returning error: {:?}", self);

        let body = json!({
            "error": error_message,
            "details": format!("{:?}", self)
        });

        (status, Json(body)).into_response()
    }
}

impl From<AppError> for StatusCode {
    fn from(error: AppError) -> Self {
        match error {
            AppError::AuthError(_) => StatusCode::UNAUTHORIZED,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::StorageError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UploadError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AIServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::FileError(_) => StatusCode::BAD_REQUEST,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
