use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use sqlx::Error as SqlxError;
use std::io::Error as IoError;
use actix_multipart::MultipartError;
use zip::result::ZipError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Authentication Error: {0}")]
    AuthenticationError(String),

    #[error("Upload Error: {0}")]
    UploadError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError(_) => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            AppError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            AppError::AuthenticationError(ref message) => HttpResponse::Unauthorized().json(message),
            AppError::UploadError(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}

impl From<SqlxError> for AppError {
    fn from(error: SqlxError) -> AppError {
        AppError::InternalServerError(error.to_string())
    }
}

impl From<IoError> for AppError {
    fn from(error: IoError) -> AppError {
        AppError::InternalServerError(error.to_string())
    }
}

impl From<MultipartError> for AppError {
    fn from(error: MultipartError) -> AppError {
        AppError::UploadError(error.to_string())
    }
}

impl From<ZipError> for AppError {
    fn from(error: ZipError) -> AppError {
        AppError::UploadError(error.to_string())
    }
}
