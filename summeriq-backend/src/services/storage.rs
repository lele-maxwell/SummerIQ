use aws_sdk_s3::{Client, primitives::ByteStream, error::SdkError};
use crate::error::AppError;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use uuid::Uuid;
use tracing::{error, info};

pub struct StorageService {
    client: Client,
    bucket: String,
}

impl StorageService {
    pub fn new(client: Client, bucket: String) -> Self {
        Self { client, bucket }
    }

    pub async fn list_buckets(&self) -> Result<Vec<String>, AppError> {
        info!("Listing MinIO buckets");
        
        let response = self.client
            .list_buckets()
            .send()
            .await
            .map_err(|e| {
                error!("Failed to list buckets: {}", e);
                match e {
                    SdkError::ServiceError(err) => {
                        AppError::StorageError(format!("MinIO service error: {}", err.err()))
                    }
                    _ => AppError::StorageError(format!("Failed to list buckets: {}", e))
                }
            })?;

        let buckets: Vec<String> = response.buckets()
            .iter()
            .filter_map(|b| b.name().map(String::from))
            .collect();

        info!("Found buckets: {:?}", buckets);
        Ok(buckets)
    }

    pub async fn upload_file(&self, file_path: &Path, key: &str) -> Result<(), AppError> {
        tracing::info!("Opening file for upload: {:?}", file_path);
        let file = match tokio::fs::File::open(file_path).await {
            Ok(file) => file,
            Err(e) => {
                tracing::error!("Failed to open file for upload: {}", e);
                return Err(AppError::FileError(format!("Failed to open file: {}", e)));
            }
        };

        let file_size = match file.metadata().await {
            Ok(metadata) => metadata.len(),
            Err(e) => {
                tracing::error!("Failed to get file metadata: {}", e);
                return Err(AppError::FileError(format!("Failed to get file metadata: {}", e)));
            }
        };
        tracing::info!("File size: {} bytes", file_size);

        let mut buffer = Vec::with_capacity(file_size as usize);
        let mut reader = tokio::io::BufReader::new(file);
        if let Err(e) = reader.read_to_end(&mut buffer).await {
            tracing::error!("Failed to read file into memory: {}", e);
            return Err(AppError::FileError(format!("Failed to read file: {}", e)));
        }
        tracing::info!("File read into memory, size: {} bytes", buffer.len());

        let body = ByteStream::from(buffer);
        tracing::info!("Attempting to upload to MinIO bucket: {} with key: {}", self.bucket, key);
        
        match self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body)
            .send()
            .await 
        {
            Ok(_) => {
                tracing::info!("Successfully uploaded file to MinIO");
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to upload file to MinIO: {}", e);
                Err(AppError::StorageError(format!("Failed to upload file to storage: {}", e)))
            }
        }
    }

    pub async fn get_file(&self, key: &str) -> Result<Vec<u8>, anyhow::Error> {
        let response = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        let bytes = response.body.collect().await?;
        Ok(bytes.to_vec())
    }

    pub async fn delete_file(&self, key: &str) -> Result<(), anyhow::Error> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        Ok(())
    }
}
