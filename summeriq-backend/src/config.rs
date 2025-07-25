use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use tracing::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub storage_path: String,
    pub groq_api_key: String,
    pub server_port: u16,
    pub upload_dir: String,
    pub post_request_delay_ms: u64,
}

impl Config {
    pub fn from_env() -> Self {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        let storage_path = current_dir.join("storage").to_string_lossy().into_owned();
        
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            storage_path: storage_path.clone(),
            groq_api_key: env::var("GROQ_API_KEY").expect("GROQ_API_KEY must be set"),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .expect("SERVER_PORT must be a number"),
            upload_dir: storage_path,
            post_request_delay_ms: env::var("POST_REQUEST_DELAY_MS").unwrap_or_else(|_| "3000".to_string()).parse().unwrap_or(3000),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl ErrorResponse {
    pub fn new(error: impl Into<String>) -> Self {
        Self { error: error.into() }
    }
}

impl Responder for ErrorResponse {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::BadRequest().json(self)
    }
}
