// src/config.rs
use std::env;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub minio_endpoint: String,
    pub minio_access_key: String,
    pub minio_secret_key: String,
    pub jwt_secret: String,
    pub openrouter_api_key: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            minio_endpoint: env::var("MINIO_ENDPOINT")?,
            minio_access_key: env::var("MINIO_ACCESS_KEY")?,
            minio_secret_key: env::var("MINIO_SECRET_KEY")?,
            jwt_secret: env::var("JWT_SECRET")?,
            openrouter_api_key: env::var("OPENROUTER_API_KEY")?,
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    message: ChatMessage,
}

pub struct AIService {
    client: Client,
    api_key: String,
}

impl AIService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String, AppError> {
        let response = self.client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&ChatRequest { messages })
            .send()
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        let chat_response = response.json::<ChatResponse>()
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(chat_response.choices[0].message.content.clone())
    }
}
