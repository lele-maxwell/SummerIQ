use actix_web::{web, HttpResponse};
use serde::Serialize;
use crate::services::{AIService, StorageService, AnalysisService};
use crate::error::AppError;
use std::fs;
use urlencoding::decode;

#[derive(Serialize)]
pub struct FileAnalysisDoc {
    pub path: String,
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub relationships: Vec<Relationship>,
}

#[derive(Serialize)]
pub struct Relationship {
    pub target_file: String,
    pub relationship_type: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct ProjectDocumentation {
    pub project_name: String,
    pub description: String,
    pub architecture: String,
    pub file_analyses: Vec<FileAnalysisDoc>,
    pub dependencies: Vec<String>,
    pub setup_instructions: String,
}

fn normalize_name(name: &str) -> String {
    name.to_lowercase().replace(|c: char| !c.is_ascii_alphanumeric(), "")
}

pub async fn get_project_documentation(
    path: web::Path<String>,
    ai_service: web::Data<AIService>,
    storage_service: web::Data<StorageService>,
    analysis_service: web::Data<AnalysisService>,
) -> Result<HttpResponse, AppError> {
    tracing::info!("get_project_documentation: incoming path: {}", path);
    let path = path.into_inner();
    let decoded_path = decode(&path).map(|c| c.to_string()).unwrap_or(path.clone());
    let trimmed = decoded_path.trim_start_matches('/');
    let path_parts: Vec<&str> = trimmed.split('/').collect();
    if path_parts.is_empty() {
        return Err(AppError::BadRequest("Empty path provided".to_string()));
    }
    let project_name = path_parts[0];
    tracing::info!("get_project_documentation: using project_name: {}", project_name);
    tracing::info!("get_project_documentation: calling get_file_id with project_name: {}", project_name);
    // Use the same logic as file analysis to get the UUID
    let uuid = storage_service.get_file_id(project_name).await?;
    let extracted_dir = format!("extracted_{}", uuid);
    let files = storage_service.list_files(&extracted_dir).await?;

    // Flatten the file tree to a list of files (not directories)
    fn flatten_files(nodes: &[crate::services::storage::FileNode], parent: &str, out: &mut Vec<(String, String)>) {
        for node in nodes {
            let full_path = if parent.is_empty() {
                node.name.clone()
            } else {
                format!("{}/{}", parent, node.name)
            };
            if node.is_dir {
                if let Some(children) = &node.children {
                    flatten_files(children, &full_path, out);
                }
            } else {
                out.push((full_path.clone(), node.name.clone()));
            }
        }
    }
    let mut file_list = Vec::new();
    flatten_files(&files, "", &mut file_list);

    let mut file_analyses = Vec::new();
    let mut all_dependencies = Vec::new();

    for (file_path, file_name) in &file_list {
        let full_path = format!("{}/{}", extracted_dir, file_path);
        if let Ok(content_bytes) = storage_service.read_file(&full_path).await {
            if let Ok(content) = String::from_utf8(content_bytes) {
                if let Ok(analysis) = analysis_service.analyze_file(file_path, &content).await {
                    all_dependencies.extend(analysis.dependencies.clone());
                    file_analyses.push(FileAnalysisDoc {
                        path: file_path.clone(),
                        name: file_name.clone(),
                        description: analysis.file_purpose,
                        dependencies: analysis.dependencies,
                        relationships: vec![], // TODO: implement relationships if needed
                    });
                }
            }
        }
    }

    // Remove duplicate dependencies
    all_dependencies.sort();
    all_dependencies.dedup();

    // Generate project description and architecture using AIService
    let description = ai_service.analyze_text(&format!(
        "Summarize the purpose and main features of the project based on the following file analyses: {:?}",
        file_analyses.iter().map(|f| &f.description).collect::<Vec<_>>()
    )).await.unwrap_or_else(|_| "No summary available.".to_string());

    let architecture = ai_service.analyze_text(&format!(
        "Describe the high-level architecture of the project based on the following files and their purposes: {:?}",
        file_analyses.iter().map(|f| format!("{}: {}", f.path, f.description)).collect::<Vec<_>>()
    )).await.unwrap_or_else(|_| "No architecture info available.".to_string());

    let setup_instructions = "No setup instructions available.".to_string(); // TODO: Optionally generate with AI

    let doc = ProjectDocumentation {
        project_name: project_name.to_string(),
        description,
        architecture,
        file_analyses,
        dependencies: all_dependencies,
        setup_instructions,
    };
    Ok(HttpResponse::Ok().json(doc))
} 