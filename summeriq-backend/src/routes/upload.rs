use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder, Scope};
use futures::{StreamExt, TryStreamExt};
use serde_json::json;
use tracing::{info, error};
use uuid::Uuid;
use chrono;
use std::io::Cursor;
use zip::ZipArchive;
use std::collections::HashMap;

use crate::error::AppError;
use crate::services::StorageService;

#[derive(Debug, Clone)]
struct FileNode {
    name: String,
    is_dir: bool,
    children: Option<Vec<FileNode>>,
}

impl FileNode {
    fn new(name: String, is_dir: bool) -> Self {
        FileNode {
            name,
            is_dir,
            children: if is_dir { Some(Vec::new()) } else { None },
        }
    }
}

fn build_file_tree(archive: &mut ZipArchive<Cursor<&Vec<u8>>>) -> Vec<FileNode> {
    let mut paths: Vec<(String, bool)> = Vec::new();
    
    // First pass: collect all paths and their types
    for i in 0..archive.len() {
        if let Ok(file) = archive.by_index(i) {
            let path = file.name().to_string();
            let is_dir = path.ends_with('/');
            let path = path.trim_end_matches('/').to_string();
            paths.push((path, is_dir));
        }
    }
    
    // Sort paths to ensure parents come before children
    paths.sort();
    
    // Second pass: build the tree
    let mut root_nodes: Vec<FileNode> = Vec::new();
    let mut node_map: HashMap<String, usize> = HashMap::new();
    
    for (path, is_dir) in paths {
        let parts: Vec<&str> = path.split('/').collect();
        let mut current_path = String::new();
        
        for (index, part) in parts.iter().enumerate() {
            let is_last = index == parts.len() - 1;
            let parent_path = current_path.clone();
            
            if current_path.is_empty() {
                current_path = part.to_string();
            } else {
                current_path = format!("{}/{}", current_path, part);
            }
            
            if !node_map.contains_key(&current_path) {
                let new_node = FileNode::new(part.to_string(), !is_last || is_dir);
                
                if parent_path.is_empty() {
                    root_nodes.push(new_node);
                    node_map.insert(current_path.clone(), root_nodes.len() - 1);
                } else if let Some(&parent_index) = node_map.get(&parent_path) {
                    if parent_index < root_nodes.len() {
                        if let Some(children) = &mut root_nodes[parent_index].children {
                            children.push(new_node);
                            node_map.insert(current_path.clone(), children.len() - 1);
                        }
                    }
                }
            }
        }
    }
    
    root_nodes
}

fn file_tree_to_json(nodes: &[FileNode]) -> Vec<serde_json::Value> {
    nodes.iter().map(|node| {
        let mut json = json!({
            "name": node.name,
            "is_dir": node.is_dir,
        });

        if let Some(children) = &node.children {
            json["children"] = json!(file_tree_to_json(children));
        }

        json
    }).collect()
}

pub async fn upload_file(
    storage_service: web::Data<StorageService>,
    mut payload: Multipart,
) -> Result<impl Responder, AppError> {
    let mut file_content = Vec::new();
    let mut filename = None;

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        filename = content_disposition.get_filename().map(|s| s.to_string());

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file_content.extend_from_slice(&data);
        }
    }

    let filename = filename.ok_or_else(|| AppError::UploadError("No filename provided".to_string()))?;
    let file_id = Uuid::new_v4();
    let final_filename = format!("{}_{}", file_id, filename);

    // Save the file
    storage_service.save_file(&file_content, &final_filename).await?;

    // Extract files from ZIP archive
    let cursor = Cursor::new(&file_content);
    let mut extracted_files = Vec::new();
    
    if let Ok(mut archive) = ZipArchive::new(cursor) {
        // Extract the ZIP file first
        let extract_dir = format!("extracted_{}", file_id);
        storage_service.extract_zip(&file_content, &extract_dir).await?;
        
        // Then build the file tree
        let file_tree = build_file_tree(&mut archive);
        extracted_files = file_tree_to_json(&file_tree);
    }

    info!("File uploaded successfully: {}", final_filename);
    Ok(HttpResponse::Created().json(json!({
        "message": "File uploaded successfully",
        "file_id": file_id,
        "filename": final_filename,
        "upload": {
            "id": file_id,
            "filename": final_filename,
            "mime_type": "application/zip",
            "created_at": chrono::Utc::now().to_rfc3339(),
            "extracted_files": extracted_files
        }
    })))
}

pub async fn get_file(
    storage_service: web::Data<StorageService>,
    file_id: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let content = storage_service.read_file(&file_id).await?;
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(content))
}

pub async fn read_file_content(
    storage_service: web::Data<StorageService>,
    file_path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let content = storage_service.read_file_content(&file_path).await?;
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(content))
}

pub async fn list_uploads() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"uploads": []}))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/upload")
            .route("", web::post().to(upload_file))
            .route("/{file_id}", web::get().to(get_file))
            .route("/content/{file_path:.*}", web::get().to(read_file_content))
    );
    cfg.route("/api/uploads", web::get().to(list_uploads));
} 