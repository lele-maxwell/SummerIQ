use std::path::{Path, PathBuf};
use tokio::fs;
use zip::ZipArchive;
use std::io::Cursor;
use tracing::{info, error};
use std::io::Read;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::future::Future;
use uuid::Uuid;
use glob;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub children: Option<Vec<FileNode>>,
}

#[derive(Clone)]
pub struct StorageService {
    upload_dir: PathBuf,
}

impl StorageService {
    pub fn new(upload_dir: String) -> Self {
        Self { 
            upload_dir: PathBuf::from(upload_dir)
        }
    }

    pub async fn save_file(&self, content: &[u8], filename: &str) -> Result<(), crate::error::AppError> {
        let file_path = self.upload_dir.join(filename);
        fs::write(&file_path, content).await?;
        info!("File saved to: {:?}", file_path);
        Ok(())
    }

    pub async fn read_file(&self, filename: &str) -> Result<Vec<u8>, crate::error::AppError> {
        let file_path = self.upload_dir.join(filename);
        info!("Attempting to read file from absolute path: {:?}", file_path);
        let content = fs::read(&file_path).await?;
        info!("Successfully read file from: {:?}", file_path);
        Ok(content)
    }

    pub async fn get_file_id(&self, project_name: &str) -> Result<String, crate::error::AppError> {
        // First try to extract UUID from project name (format: UUID_project)
        let parts: Vec<&str> = project_name.split('_').collect();
        if parts.len() >= 2 {
            // If the project name contains a UUID, return it
            return Ok(parts[0].to_string());
        }
        
        // If no UUID in project name, try to find it in the storage directory
        let dir = self.upload_dir.join("extracted_*");
        let pattern = dir.to_string_lossy();
        let entries = glob::glob(&pattern)
            .map_err(|e| crate::error::AppError::InternalServerError(format!("Failed to search for UUID: {}", e)))?;
        
        for entry in entries {
            if let Ok(path) = entry {
                if let Some(file_name) = path.file_name() {
                    let name = file_name.to_string_lossy();
                    if name.starts_with("extracted_") {
                        let uuid = name.trim_start_matches("extracted_");
                        return Ok(uuid.to_string());
                    }
                }
            }
        }
        
        Err(crate::error::AppError::BadRequest("No UUID found for project".to_string()))
    }

    pub async fn extract_zip(&self, content: &[u8], base_filename: &str) -> Result<Vec<String>, crate::error::AppError> {
        let cursor = Cursor::new(content);
        let mut archive = ZipArchive::new(cursor)?;
        let extract_dir = self.upload_dir.join(base_filename);
        
        // Create the extraction directory
        fs::create_dir_all(&extract_dir).await?;
        
        let mut extracted_files = Vec::new();
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = extract_dir.join(file.name());
            
            if file.name().ends_with('/') {
                // Create directory
                fs::create_dir_all(&outpath).await?;
            } else {
                // Extract file
                if let Some(parent) = outpath.parent() {
                    fs::create_dir_all(parent).await?;
                }
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;
                fs::write(&outpath, buffer).await?;
            }
            
            extracted_files.push(file.name().to_string());
        }
        
        info!("ZIP file extracted to: {:?}", extract_dir);
        Ok(extracted_files)
    }

    pub async fn list_files(&self, dir: &str) -> Result<Vec<FileNode>, crate::error::AppError> {
        let dir_path = self.upload_dir.join(dir);
        let mut root_nodes = Vec::new();
        
        async fn process_directory(path: &Path, base_path: &Path) -> Result<FileNode, std::io::Error> {
            let relative_path = path.strip_prefix(base_path).unwrap();
            let path_str = relative_path.to_string_lossy().into_owned();
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            
            let mut node = FileNode {
                name,
                path: path_str,
                is_dir: path.is_dir(),
                children: None,
            };
            
            if path.is_dir() {
                let mut entries = fs::read_dir(path).await?;
                let mut children = Vec::new();
                
                while let Some(entry) = entries.next_entry().await? {
                    let entry_path = entry.path();
                    let child_node = Box::pin(process_directory(&entry_path, base_path)).await?;
                    children.push(child_node);
                }
                
                // Sort children: directories first, then files, both alphabetically
                children.sort_by(|a, b| {
                    match (a.is_dir, b.is_dir) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => a.name.cmp(&b.name),
                    }
                });
                
                node.children = Some(children);
            }
            
            Ok(node)
        }
        
        if dir_path.is_dir() {
            let mut entries = fs::read_dir(&dir_path).await?;
            while let Some(entry) = entries.next_entry().await? {
                let entry_path = entry.path();
                let node = Box::pin(process_directory(&entry_path, &dir_path)).await?;
                root_nodes.push(node);
            }
            
            // Sort root nodes: directories first, then files, both alphabetically
            root_nodes.sort_by(|a, b| {
                match (a.is_dir, b.is_dir) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name.cmp(&b.name),
                }
            });
        }
        
        Ok(root_nodes)
    }
} 