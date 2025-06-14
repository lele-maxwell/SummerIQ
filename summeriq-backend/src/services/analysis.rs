use std::path::Path;
use tracing::{info, error};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::error::AppError;
use reqwest::Client;
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileAnalysis {
    pub language: String,
    pub components: Vec<String>,
    pub dependencies: Vec<String>,
    pub recommendations: Vec<String>,
    pub analysis_time: String,
}

pub struct AnalysisService {
    client: Client,
    api_key: String,
}

impl AnalysisService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn analyze_file(&self, file_path: &str, content: &str) -> Result<FileAnalysis, AppError> {
        info!("Sending request to OpenRouter API");
        
        let prompt = format!(
            "Analyze this code file and provide a detailed analysis in JSON format. Include the following fields:\n\
            - language: The programming language used\n\
            - components: List of key components or functions\n\
            - dependencies: List of external dependencies\n\
            - recommendations: List of improvement suggestions\n\
            - analysis_time: Current timestamp in ISO format\n\n\
            File path: {}\n\n\
            Code content:\n{}",
            file_path, content
        );

        let response = self.client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://github.com/yourusername/summeriq")
            .header("X-Title", "SummerIQ")
            .json(&json!({
                "model": "anthropic/claude-3-opus:beta",
                "messages": [
                    {
                        "role": "user",
                        "content": prompt
                    }
                ],
                "max_tokens": 300
            }))
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send request to OpenRouter API: {}", e);
                AppError::AnalysisError("Failed to send request to AI service".into())
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            error!("AI service error: {} - {}", status, error_text);
            return Err(AppError::AnalysisError(format!("AI service error: {}", status)));
        }

        let response_text = response.text().await.map_err(|e| {
            error!("Failed to read response from OpenRouter API: {}", e);
            AppError::AnalysisError("Failed to read response from AI service".into())
        })?;

        info!("Received response from OpenRouter API: {}", response_text);

        // Parse the OpenRouter response format
        let response_json: serde_json::Value = serde_json::from_str(&response_text).map_err(|e| {
            error!("Failed to parse OpenRouter response: {}", e);
            AppError::AnalysisError("Failed to parse AI service response".into())
        })?;

        // Extract the content from the response
        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| {
                error!("Invalid response format: missing content field");
                AppError::AnalysisError("Invalid response format from AI service".into())
            })?;

        info!("Extracted content from response: {}", content);

        // Clean up the content by removing any trailing incomplete JSON
        let cleaned_content = if let Some(last_brace) = content.rfind('}') {
            &content[..=last_brace]
        } else {
            content
        };

        // Parse the analysis content
        let mut analysis: FileAnalysis = serde_json::from_str(cleaned_content).map_err(|e| {
            error!("Failed to parse analysis JSON: {}", e);
            AppError::AnalysisError("Failed to parse analysis from AI service".into())
        })?;

        // Ensure analysis_time is set
        if analysis.analysis_time.is_empty() {
            analysis.analysis_time = Utc::now().to_rfc3339();
        }

        Ok(analysis)
    }
} 