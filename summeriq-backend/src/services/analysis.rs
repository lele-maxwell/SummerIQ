use std::path::Path;
use tracing::{info, error};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::error::AppError;
use reqwest::Client;
use serde_json::json;
use std::fs;
use crate::services::StorageService;
use crate::services::ai::AIService;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use sha2::{Sha256, Digest};
use regex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileAnalysis {
    pub language: String,
    pub file_purpose: String,
    pub dependencies: Vec<String>,
    pub analysis_time: String,
    pub contents: String,
}

pub struct AnalysisService {
    client: Client,
    api_key: String,
    storage_service: StorageService,
    ai_service: AIService,
}

// Global cache: key = hash(file_path + content), value = FileAnalysis
static FILE_ANALYSIS_CACHE: Lazy<DashMap<String, FileAnalysis>> = Lazy::new(DashMap::new);

// Helper function to clean up AI responses by removing thinking process markers
fn clean_ai_response(response: &str) -> String {
    let mut cleaned = response.to_string();
    
    // Remove <think>...</think> blocks
    let think_pattern = regex::Regex::new(r#"<think>.*?</think>"#).unwrap();
    cleaned = think_pattern.replace_all(&cleaned, "").to_string();
    
    // Remove <reasoning>...</reasoning> blocks
    let reasoning_pattern = regex::Regex::new(r#"<reasoning>.*?</reasoning>"#).unwrap();
    cleaned = reasoning_pattern.replace_all(&cleaned, "").to_string();
    
    // Remove <analysis>...</analysis> blocks
    let analysis_pattern = regex::Regex::new(r#"<analysis>.*?</analysis>"#).unwrap();
    cleaned = analysis_pattern.replace_all(&cleaned, "").to_string();
    
    // Remove any remaining thinking markers at the start
    let thinking_start_pattern = regex::Regex::new(r#"^<[^>]*>.*?</[^>]*>\s*"#).unwrap();
    cleaned = thinking_start_pattern.replace_all(&cleaned, "").to_string();
    
    // Clean up extra whitespace and newlines
    cleaned = cleaned.trim().to_string();
    
    // Remove any leading/trailing whitespace and normalize newlines
    cleaned = cleaned.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    
    cleaned
}

impl AnalysisService {
    pub fn new(api_key: String, storage_service: StorageService, ai_service: AIService) -> Self {
        info!("Initializing AnalysisService");
        Self {
            client: Client::new(),
            api_key,
            storage_service,
            ai_service,
        }
    }

    pub async fn analyze_file(&self, file_path: &str, content: &str) -> Result<FileAnalysis, AppError> {
        // Compute hash of file_path + content for cache key
        let mut hasher = Sha256::new();
        hasher.update(file_path.as_bytes());
        hasher.update(content.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        // Check cache
        if let Some(cached) = FILE_ANALYSIS_CACHE.get(&hash) {
            info!("Cache hit for file analysis: {}", file_path);
            return Ok(cached.clone());
        }
        info!("Cache miss for file analysis: {}", file_path);
        info!("Starting file analysis for: {}", file_path);
        info!("Content length: {} bytes", content.len());
        
        // Generate file purpose analysis
        let purpose_prompt = format!(
            "Analyze this code file and provide a brief explanation of its main purpose and contents. \
             Focus on explaining what the file does in the context of the project. \
             Keep the explanation concise but informative. \
             CRITICAL: Provide ONLY a direct, factual answer. Do NOT include any thinking process, internal monologue, reasoning steps, or meta-commentary. \
             Do NOT start with '<think>', '<reasoning>', or any similar markers. \
             Do NOT explain your analysis process. \
             Simply state what the file is and what it does. \
             Example format: 'This is a configuration file that...' or 'This file implements...'\n\
             \nFile content:\n{}",
            content
        );

        info!("Sending purpose analysis request");
        let file_purpose = self.ai_service
            .analyze_text(&purpose_prompt)
            .await?;
        info!("Received purpose analysis: {}", file_purpose);

        // Generate dependencies analysis
        let deps_prompt = format!(
            "Analyze this code file and list all its dependencies (imports, requires, etc.). \
             If there are no dependencies, just say 'No dependencies found.' \
             Format each dependency on a new line, starting with a dash (-). \
             For each dependency, include its purpose if it's not obvious from the name. \
             CRITICAL: Provide ONLY a direct, factual answer. Do NOT include any thinking process, internal monologue, reasoning steps, or meta-commentary. \
             Do NOT start with '<think>', '<reasoning>', or any similar markers. \
             Do NOT explain your analysis process. \
             Simply list the dependencies or state 'No dependencies found.' \
             Example format:\n\
             - react: Frontend UI library\n\
             - express: Web server framework\n\
             No dependencies found\n\
             \nFile content:\n{}",
            content
        );

        info!("Sending dependencies analysis request");
        let dependencies_text = self.ai_service
            .analyze_text(&deps_prompt)
            .await?;
        info!("Received dependencies analysis: {}", dependencies_text);

        // Clean the dependencies response and parse into a vector
        let cleaned_deps_text = clean_ai_response(&dependencies_text);
        let dependencies: Vec<String> = cleaned_deps_text
            .lines()
            .filter(|line| line.trim().starts_with('-'))
            .map(|line| line.trim_start_matches('-').trim().to_string())
            .collect();

        info!("Parsed {} dependencies from the file", dependencies.len());

        // Detect language from file extension
        let language = Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_string();

        info!("Detected language: {}", language);

        let analysis = FileAnalysis {
            language,
            file_purpose: clean_ai_response(&file_purpose),
            dependencies,
            analysis_time: Utc::now().to_rfc3339(),
            contents: content.to_string(),
        };
        // Store in cache
        FILE_ANALYSIS_CACHE.insert(hash, analysis.clone());
        info!("Analysis complete: {:?}", analysis);
        Ok(analysis)
    }
} 