use actix_web::{web, HttpResponse};
use tracing::{info, error};
use crate::services::analysis::AnalysisService;
use crate::services::StorageService;
use crate::error::AppError;

pub async fn analyze_file(
    analysis_service: web::Data<AnalysisService>,
    storage_service: web::Data<StorageService>,
    file_path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    info!("Received analysis request for file: {}", file_path);
    
    // Get the file ID from the path
    let path_parts: Vec<&str> = file_path.split('/').collect();
    if path_parts.len() < 2 {
        return Err(AppError::BadRequest("Invalid file path".to_string()));
    }
    
    // Get the project name and the rest of the path
    let project_name = path_parts[1]; // Skip the first empty part
    let relative_path = path_parts[2..].join("/");
    
    // Get the file ID from the storage service
    let file_id = storage_service.get_file_id(project_name).await?;
    let full_path = format!("extracted_{}/{}", file_id, relative_path);
    
    info!("Analyzing file at path: {}", full_path);
    
    // Read the file content
    let content = storage_service.read_file(&full_path).await?;
    let content_str = String::from_utf8(content)
        .map_err(|e| AppError::InternalServerError(format!("Invalid UTF-8 content: {}", e)))?;
    
    // Analyze the file
    let analysis = analysis_service.analyze_file(&full_path, &content_str).await?;
    
    Ok(HttpResponse::Ok().json(analysis))
} 