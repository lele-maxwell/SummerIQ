use actix_web::{web, HttpResponse, Responder};
use tracing::info;
use urlencoding::decode;
use sqlx::PgPool;

use crate::services::AnalysisService;
use crate::config::Config;
use crate::services::StorageService;

pub async fn analyze_file(
    path: web::Path<String>,
    analysis_service: web::Data<AnalysisService>,
    storage_service: web::Data<StorageService>,
    config: web::Data<Config>,
    pool: web::Data<PgPool>,
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
    
    // Get the most recent upload for this project
    let upload = match sqlx::query!(
        r#"
        SELECT id, filename
        FROM uploads
        WHERE original_filename LIKE $1
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        format!("%{}%", project)
    )
    .fetch_optional(&**pool)
    .await
    {
        Ok(Some(upload)) => upload,
        Ok(None) => {
            info!("No upload found for project: {}", project);
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "No upload found for this project",
                "details": format!("No upload found for project: {}", project)
            }));
        }
        Err(e) => {
            info!("Database error: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error",
                "details": e.to_string()
            }));
        }
    };
    
    // Extract the UUID from the filename
    let file_id = upload.id;
    
    // Construct the full path with storage directory and UUID
    let full_path = format!("extracted_{}/{}", file_id, inner_path);
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