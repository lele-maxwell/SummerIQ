use std::path::{Path, PathBuf};
use tokio::fs;
use zip::ZipArchive;
use std::io::Cursor;
use tracing::{info, error};
use std::io::Read;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::future::Future;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub children: Option<Vec<FileNode>>,
}

pub struct StorageService {
    upload_dir: String,
}

impl StorageService {
    pub fn new(upload_dir: String) -> Self {
        Self { upload_dir }
    }

    pub async fn save_file(&self, content: &[u8], filename: &str) -> Result<(), crate::error::AppError> {
        let file_path = Path::new(&self.upload_dir).join(filename);
        fs::write(&file_path, content).await?;
        info!("File saved to: {:?}", file_path);
        Ok(())
    }

    pub async fn read_file(&self, filename: &str) -> Result<Vec<u8>, crate::error::AppError> {
        let file_path = Path::new(&self.upload_dir).join(filename);
        let content = fs::read(&file_path).await?;
        info!("File read from: {:?}", file_path);
        Ok(content)
    }

    pub async fn extract_zip(&self, content: &[u8], extract_dir: &str) -> Result<(), crate::error::AppError> {
        let extract_path = Path::new(&self.upload_dir).join(extract_dir);
        fs::create_dir_all(&extract_path).await?;

        let cursor = Cursor::new(content);
        let mut archive = ZipArchive::new(cursor)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = extract_path.join(file.name());

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath).await?;
            } else {
                if let Some(parent) = outpath.parent() {
                    fs::create_dir_all(parent).await?;
                }
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;
                fs::write(&outpath, buffer).await?;
            }
        }

        info!("ZIP file extracted to: {:?}", extract_path);
        Ok(())
    }

    pub async fn list_files(&self, dir: &str) -> Result<Vec<FileNode>, crate::error::AppError> {
        let dir_path = Path::new(&self.upload_dir).join(dir);
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