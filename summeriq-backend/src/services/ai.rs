use reqwest::Client;
use serde_json::{json, Value};
use tracing::{info, error, warn};
use std::sync::Arc;
use tokio::sync::Mutex;
use once_cell::sync::Lazy;
use tokio::time::{sleep, Duration};
use regex::Regex;

use crate::error::AppError;

// Add a global mutex for throttling Groq API requests
static AI_THROTTLE: Lazy<Arc<Mutex<()>>> = Lazy::new(|| Arc::new(Mutex::new(())));

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
        let _throttle = AI_THROTTLE.lock().await;
        let client = reqwest::Client::new();
        let mut retries = 0;
        loop {
            let response = client
                .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
            .json(&json!({
                    "model": "deepseek-r1-distill-llama-70b",
                "messages": [
                        {"role": "system", "content": "You are a helpful AI assistant that analyzes code and provides clear, concise responses. IMPORTANT: Do not include any thinking process, internal monologue, or meta-commentary in your response. Do not start with '<think>' or similar markers. Provide direct, factual answers only."},
                        {"role": "user", "content": prompt}
                    ],
                    "temperature": 0.7,
                    "max_tokens": 1000
                }))
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
                if status.as_u16() == 429 {
                    // Parse wait time from error message
                    let re = Regex::new(r#"try again in ([\d.]+)ms|try again in ([\d.]+)s"#).unwrap();
                    if let Some(caps) = re.captures(&error_text) {
                        let wait_ms = if let Some(ms) = caps.get(1) {
                            ms.as_str().parse::<f64>().unwrap_or(1.0)
                        } else if let Some(s) = caps.get(2) {
                            s.as_str().parse::<f64>().unwrap_or(1.0) * 1000.0
                        } else {
                            1000.0
                        };
                        let wait = std::time::Duration::from_millis(wait_ms as u64);
                        warn!("Rate limited. Waiting {:?} before retrying... (attempt {}/{})", wait, retries+1, 5);
                        tokio::time::sleep(wait).await;
                        retries += 1;
                        if retries < 5 {
                            continue;
                        }
                    }
                    return Err(AppError::BadRequest("AI service is rate limited. Please try again later.".to_string()));
                }
                if status.as_u16() == 413 {
                    return Err(AppError::BadRequest("AI request too large. Please reduce the size of your request.".to_string()));
                }
                let error_message = match status.as_u16() {
                    503 => "AI service is temporarily unavailable. Please try again in a few moments.".to_string(),
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
            // Add a 3-second delay after every successful AI call
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            return Ok(content.to_string());
        }
    }
} 