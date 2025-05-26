use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use std::env;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error")]
    AuthError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("AI service error: {0}")]
    AIServiceError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Internal server error")]
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::AuthError(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::StorageError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::AIServiceError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = json!({
            "error": error_message
        });

        (status, Json(body)).into_response()
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub jwt_secret: String,
    pub minio_endpoint: String,
    pub minio_access_key: String,
    pub minio_secret_key: String,
    pub openrouter_api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        tracing::debug!("Loading configuration from environment variables");
        
        let database_url = env::var("DATABASE_URL")
            .map_err(|e| {
                tracing::error!("Failed to load DATABASE_URL: {}", e);
                e
            })?;
            
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);
            
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|e| {
                tracing::error!("Failed to load JWT_SECRET: {}", e);
                e
            })?;
            
        let minio_endpoint = env::var("MINIO_ENDPOINT")
            .map_err(|e| {
                tracing::error!("Failed to load MINIO_ENDPOINT: {}", e);
                e
            })?;
            
        let minio_access_key = env::var("MINIO_ACCESS_KEY")
            .map_err(|e| {
                tracing::error!("Failed to load MINIO_ACCESS_KEY: {}", e);
                e
            })?;
            
        let minio_secret_key = env::var("MINIO_SECRET_KEY")
            .map_err(|e| {
                tracing::error!("Failed to load MINIO_SECRET_KEY: {}", e);
                e
            })?;
            
        let openrouter_api_key = env::var("OPENROUTER_API_KEY")
            .map_err(|e| {
                tracing::error!("Failed to load OPENROUTER_API_KEY: {}", e);
                e
            })?;

        tracing::debug!("Successfully loaded all configuration variables");
        
        Ok(Self {
            database_url,
            server_port,
            jwt_secret,
            minio_endpoint,
            minio_access_key,
            minio_secret_key,
            openrouter_api_key,
        })
    }
}

impl IntoResponse for Config {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::OK;
        let body = serde_json::json!({
            "database_url": self.database_url,
            "server_port": self.server_port,
            "minio_endpoint": self.minio_endpoint,
        });
        (status, axum::Json(body)).into_response()
    }
}
