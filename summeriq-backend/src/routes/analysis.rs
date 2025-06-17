use actix_web::{web, HttpResponse, Responder};
use tracing::info;
use urlencoding::decode;

use crate::services::AnalysisService;
use crate::config::Config;
use crate::services::StorageService;

pub async fn analyze_file(
    path: web::Path<String>,
    analysis_service: web::Data<AnalysisService>,
    storage_service: web::Data<StorageService>,
    config: web::Data<Config>,
) -> impl Responder {
    info!("Analyzing file: {}", path);
    
    // Decode the URL-encoded path
    let decoded_path = match decode(&path) {
        Ok(path) => path,
        Err(e) => {
            info!("Error decoding path: {}", e);
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid path encoding",
                "details": e.to_string()
            }));
        }
    };
    
    // Remove leading slash and split into project and inner path
    let trimmed = decoded_path.trim_start_matches('/');
    let mut parts = trimmed.splitn(2, '/');
    let project = parts.next().unwrap_or("");
    let inner_path = parts.next().unwrap_or("");
    
    // Use the UUID from the file content endpoint
    let uuid = "4601080f-380d-4c3e-bebb-0728e78043f9";
    
    // Construct the full path with storage directory and UUID
    // If inner_path is empty, use the filename from the URL path
    let full_path = if inner_path.is_empty() {
        format!("extracted_{}/{}", uuid, project)
    } else {
        // If the inner_path starts with the project name, remove it
        let path = if inner_path.starts_with(&format!("{}/", project)) {
            inner_path.trim_start_matches(&format!("{}/", project))
        } else {
            inner_path
        };
        format!("extracted_{}/{}", uuid, path)
    };
    info!("Full path for analysis: {}", full_path);
    
    match analysis_service.analyze_file(&full_path).await {
        Ok(analysis) => HttpResponse::Ok().json(analysis),
        Err(e) => {
            info!("Error analyzing file: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to analyze file",
                "details": e.to_string()
            }))
        }
    }
} 