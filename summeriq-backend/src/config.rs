use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use tracing::{info, error};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub storage_path: PathBuf,
    pub openrouter_api_key: String,
    pub server_port: u16,
    pub upload_dir: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            jwt_secret: env::var("JWT_SECRET")?,
            storage_path: PathBuf::from(env::var("STORAGE_PATH").unwrap_or_else(|_| "storage".to_string())),
            openrouter_api_key: env::var("OPENROUTER_API_KEY")?,
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("SERVER_PORT must be a number"),
            upload_dir: env::var("UPLOAD_DIR").expect("UPLOAD_DIR must be set"),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl ErrorResponse {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}

impl Responder for ErrorResponse {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::BadRequest().json(self)
    }
}
