use std::path::Path;
use tracing::{info, error};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::error::AppError;
use reqwest::Client;
use serde_json::json;
use std::fs;
use crate::services::StorageService;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileAnalysis {
    pub language: String,
    pub components: Vec<String>,
    pub dependencies: Vec<String>,
    pub recommendations: Vec<String>,
    pub analysis_time: String,
    pub contents: String,
}

pub struct AnalysisService {
    client: Client,
    api_key: String,
    storage_service: StorageService,
}

impl AnalysisService {
    pub fn new(api_key: String, storage_service: StorageService) -> Self {
        Self {
            client: Client::new(),
            api_key,
            storage_service,
        }
    }

    pub async fn analyze_file(&self, file_path: &str) -> Result<FileAnalysis, AppError> {
        info!("Analyzing file: {}", file_path);
        
        // Read file content from storage
        let content = self.storage_service.read_file(file_path).await
            .map_err(|e| AppError::InternalServerError(format!("Failed to read file: {}", e)))?;
        
        // Convert content to string
        let content_str = String::from_utf8(content)
            .map_err(|e| AppError::InternalServerError(format!("Invalid UTF-8 content: {}", e)))?;
        
        // Return analysis with file contents
        Ok(FileAnalysis {
            language: "rust".to_string(),
            components: vec!["main".to_string()],
            dependencies: vec!["actix-web".to_string()],
            recommendations: vec!["Add error handling".to_string()],
            analysis_time: Utc::now().to_rfc3339(),
            contents: content_str,
        })
    }
} 