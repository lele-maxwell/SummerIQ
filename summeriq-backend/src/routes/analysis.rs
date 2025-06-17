use actix_web::{web, HttpResponse, Responder};
use tracing::info;
use urlencoding::decode;
use std::fs;

use crate::services::AnalysisService;
use crate::config::Config;
use crate::services::StorageService;
use crate::services::ai::AIService;
use crate::error::AppError;

pub async fn analyze_file(
    path: web::Path<String>,
    analysis_service: web::Data<AnalysisService>,
    storage_service: web::Data<StorageService>,
    ai_service: web::Data<AIService>,
    config: web::Data<Config>,
) -> Result<impl Responder, AppError> {
    info!("Analyzing file: {}", path);
    
    // Decode the URL-encoded path
    let decoded_path = match decode(&path) {
        Ok(path) => path,
        Err(e) => {
            info!("Error decoding path: {}", e);
            return Err(AppError::BadRequest(format!("Invalid path encoding: {}", e)));
        }
    };
    
    // Remove leading slash and get the project name from the path
    let trimmed = decoded_path.trim_start_matches('/');
    let path_parts: Vec<&str> = trimmed.split('/').collect();
    
    if path_parts.is_empty() {
        return Err(AppError::BadRequest("Empty path provided".to_string()));
    }
    
    // Get the project name (first part) and the rest of the path
    let project = path_parts[0];
    let inner_path = path_parts[1..].join("/");
    
    // Get the UUID from the storage service
    let uuid = storage_service.get_file_id(project).await?;
    
    // Construct the full path with storage directory and UUID
    let full_path = if inner_path.is_empty() {
        format!("extracted_{}/{}", uuid, project)
    } else {
        format!("extracted_{}/{}", uuid, inner_path)
    };
    info!("Full path for analysis: {}", full_path);

    // Read the file content
    let content = storage_service.read_file(&full_path).await?;
    let file_content = String::from_utf8(content)
        .map_err(|e| AppError::InternalServerError(format!("Failed to read file content: {}", e)))?;

    // Analyze the file using the AnalysisService
    let analysis = analysis_service.analyze_file(&full_path, &file_content).await?;

    Ok(HttpResponse::Ok().json(analysis))
} 