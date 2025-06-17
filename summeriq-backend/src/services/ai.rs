use reqwest::Client;
use serde_json::json;
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

    pub async fn analyze_text(&self, text: &str) -> Result<String, AppError> {
        info!("Making AI API request with text length: {}", text.len());
        
        let request_body = json!({
            "model": "anthropic/claude-3-opus-20240229",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a code analysis expert. Your task is to analyze code files and provide clear, concise explanations. Focus on the main purpose and functionality of the code. When analyzing dependencies, list them in a clear, structured format."
                },
                {
                    "role": "user",
                    "content": text
                }
            ],
            "temperature": 0.7,
            "max_tokens": 300
        });

        info!("Request body: {:?}", request_body);

        let response = self.client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://github.com/yourusername/summeriq")
            .header("X-Title", "SummerIQ")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to send AI request: {}", e)))?;

        info!("Received response with status: {}", response.status());

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            error!("API error response: {}", error_text);
            return Err(AppError::InternalServerError(format!(
                "AI API error: {} - {}",
                status,
                error_text
            )));
        }

        let result = response.json::<serde_json::Value>().await
            .map_err(|e| AppError::InternalServerError(format!("Failed to parse AI response: {}", e)))?;
        info!("Full API Response: {:?}", result);
        
        let content = result["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| {
                error!("No content in API response: {:?}", result);
                AppError::InternalServerError("No content in API response".to_string())
            })?;
            
        info!("Extracted content: {}", content);
        
        Ok(content.to_string())
    }
} 