use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use zip::ZipArchive;
use std::io::Cursor;
use mime_guess::from_path;
use tracing::{info, error, debug, warn};

use crate::error::AppError;

const MAX_FILE_SIZE: usize = 20 * 1024 * 1024; // 20MB
const ALLOWED_MIME_TYPES: &[&str] = &[
    "image/jpeg",
    "image/png",
    "image/gif",
    "application/pdf",
    "text/plain",
    "application/zip",
];

#[derive(Debug)]
pub struct StorageService {
    base_path: PathBuf,
}

#[derive(Debug, serde::Serialize)]
pub struct FileMetadata {
    pub original_filename: String,
    pub stored_path: String,
    pub mime_type: String,
    pub size: usize,
    pub is_zip: bool,
    pub extracted_files: Option<Vec<String>>,
}

impl StorageService {
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    /// Upload a file and return its metadata
    pub async fn upload_file_bytes(&self, data: Vec<u8>, original_filename: &str) -> Result<FileMetadata, AppError> {
        // Validate file size
        if data.len() > MAX_FILE_SIZE {
            return Err(AppError::UploadError(format!(
                "File too large. Maximum size is {} bytes",
                MAX_FILE_SIZE
            )));
        }

        // Determine file type and storage path
        let mime_type = from_path(original_filename)
            .first_or_octet_stream()
            .to_string();

        // Validate MIME type
        if !ALLOWED_MIME_TYPES.contains(&mime_type.as_str()) {
            return Err(AppError::UploadError(format!(
                "Unsupported file type: {}. Allowed types: {:?}",
                mime_type, ALLOWED_MIME_TYPES
            )));
        }

        let is_zip = mime_type == "application/zip";
        let (stored_path, extracted_files) = if is_zip {
            self.handle_zip_upload(data, original_filename).await?
        } else {
            (self.store_raw_file(data, original_filename).await?, None)
        };

        Ok(FileMetadata {
            original_filename: original_filename.to_string(),
            stored_path: stored_path.to_string_lossy().to_string(),
            mime_type,
            size: data.len(),
            is_zip,
            extracted_files,
        })
    }

    /// Store a raw (non-zip) file
    async fn store_raw_file(&self, data: Vec<u8>, original_filename: &str) -> Result<PathBuf, AppError> {
        let file_id = Uuid::new_v4();
        let extension = Path::new(original_filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("bin");

        let stored_filename = format!("{}.{}", file_id, extension);
        let stored_path = self.base_path.join("raw_files").join(stored_filename);

        // Ensure directory exists
        if let Some(parent) = stored_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                error!("Failed to create directory: {:?}", e);
                AppError::StorageError(format!("Failed to create directory: {}", e))
            })?;
        }

        // Write file
        let mut file = File::create(&stored_path).await.map_err(|e| {
            error!("Failed to create file: {:?}", e);
            AppError::StorageError(format!("Failed to create file: {}", e))
        })?;

        file.write_all(&data).await.map_err(|e| {
            error!("Failed to write file: {:?}", e);
            AppError::StorageError(format!("Failed to write file: {}", e))
        })?;

        Ok(stored_path)
    }

    /// Handle zip file upload, including extraction
    async fn handle_zip_upload(&self, data: Vec<u8>, original_filename: &str) -> Result<(PathBuf, Option<Vec<String>>), AppError> {
        // First, store the original zip file
        let zip_path = self.store_raw_file(data.clone(), original_filename).await?;

        // Create a unique directory for extracted contents
        let extract_id = Uuid::new_v4();
        let extract_dir = self.base_path.join("extracted_content").join(extract_id.to_string());

        // Ensure extraction directory exists
        fs::create_dir_all(&extract_dir).await.map_err(|e| {
            error!("Failed to create extraction directory: {:?}", e);
            AppError::StorageError(format!("Failed to create extraction directory: {}", e))
        })?;

        // Extract zip contents
        let mut extracted_files = Vec::new();
        let cursor = Cursor::new(data);
        let mut archive = ZipArchive::new(cursor).map_err(|e| {
            error!("Failed to read zip archive: {:?}", e);
            AppError::UploadError(format!("Invalid zip file: {}", e))
        })?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| {
                error!("Failed to read zip entry {}: {:?}", i, e);
                AppError::UploadError(format!("Failed to read zip entry: {}", e))
            })?;

            // Skip directories
            if file.name().ends_with('/') {
                continue;
            }

            // Sanitize the file path to prevent path traversal
            let sanitized_path = self.sanitize_path(file.name());
            let target_path = extract_dir.join(&sanitized_path);

            // Ensure parent directory exists
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent).await.map_err(|e| {
                    error!("Failed to create directory for extracted file: {:?}", e);
                    AppError::StorageError(format!("Failed to create directory: {}", e))
                })?;
            }

            // Extract the file
            let mut outfile = File::create(&target_path).await.map_err(|e| {
                error!("Failed to create extracted file: {:?}", e);
                AppError::StorageError(format!("Failed to create file: {}", e))
            })?;

            std::io::copy(&mut file, &mut outfile).map_err(|e| {
                error!("Failed to extract file: {:?}", e);
                AppError::StorageError(format!("Failed to extract file: {}", e))
            })?;

            extracted_files.push(sanitized_path.to_string_lossy().to_string());
        }

        Ok((zip_path, Some(extracted_files)))
    }

    /// Sanitize a file path to prevent path traversal
    fn sanitize_path(&self, path: &str) -> PathBuf {
        let path = path.replace('\\', "/");
        let components: Vec<_> = path
            .split('/')
            .filter(|c| !c.is_empty() && c != ".." && c != ".")
            .collect();
        components.iter().collect()
    }
}
