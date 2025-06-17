use reqwest::Client;
use serde_json::{json, Value};
use tracing::{info, error, warn};

use crate::error::AppError;

#[derive(Clone)]
pub struct AIService {
    client: Client,
    api_key: String,
}

impl AIService {
    pub fn new(api_key: String) -> Self {
        info!("Initializing AIService with API key: {}...", api_key.chars().take(4).collect::<String>());
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn analyze_text(&self, prompt: &str) -> Result<String, AppError> {
        let client = reqwest::Client::new();
        
        let request_body = json!({
            "model": "deepseek-r1-distill-llama-70b",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful AI assistant that analyzes code and provides clear, concise responses. IMPORTANT: Do not include any thinking process, internal monologue, or meta-commentary in your response. Do not start with '<think>' or similar markers. Provide direct, factual answers only."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.7,
            "max_tokens": 1000
        });

        info!("Sending request to Groq API");
        let response = client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send request to Groq API: {}", e);
                AppError::InternalServerError(format!("Failed to connect to AI service: {}", e))
            })?;

        info!("Received response with status: {}", response.status());
        let status = response.status();
        
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            error!("API error response: {}", error_text);
            
            let error_message = match status.as_u16() {
                503 => "AI service is temporarily unavailable. Please try again in a few moments.".to_string(),
                429 => "Rate limit exceeded. Please try again later.".to_string(),
                401 => "Invalid API key. Please check your configuration.".to_string(),
                _ => format!("AI service error: {} - {}", status, error_text)
            };
            
            return Err(AppError::InternalServerError(error_message));
        }

        let response_body = response.json::<Value>().await.map_err(|e| {
            error!("Failed to parse API response: {}", e);
            AppError::InternalServerError("Failed to parse AI service response".to_string())
        })?;

        let content = response_body["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| {
                error!("Invalid response format from AI service");
                AppError::InternalServerError("Invalid response from AI service".to_string())
            })?;

        Ok(content.to_string())
    }
} 