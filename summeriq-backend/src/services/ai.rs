use reqwest::Client;
use serde_json::json;
use tracing::{info, error};

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

    pub async fn analyze_text(&self, text: &str) -> Result<String, reqwest::Error> {
        let response = self.client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://github.com/yourusername/summeriq")
            .header("X-Title", "SummerIQ")
            .json(&json!({
                "model": "anthropic/claude-3-opus-20240229",
                "messages": [
                    {
                        "role": "system",
                        "content": "You are a helpful assistant that analyzes text."
                    },
                    {
                        "role": "user",
                        "content": text
                    }
                ]
            }))
            .send()
            .await?;

        let result = response.json::<serde_json::Value>().await?;
        Ok(result["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string())
    }
} 