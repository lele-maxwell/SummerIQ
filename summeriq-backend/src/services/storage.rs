use aws_sdk_s3::{Client, primitives::ByteStream, error::SdkError};
use crate::error::AppError;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use uuid::Uuid;
use tracing::{error, info, debug};

pub struct StorageService {
    client: Client,
    bucket: String,
}

impl StorageService {
    pub fn new(client: Client, bucket: String) -> Self {
        info!("Creating StorageService with bucket: {}", bucket);
        Self { client, bucket }
    }

    pub fn bucket_name(&self) -> &str {
        &self.bucket
    }

    async fn ensure_bucket_exists(&self) -> Result<(), AppError> {
        info!("Checking if bucket exists: {}", self.bucket);
        
        // First try to list buckets to see if ours exists
        match self.client.list_buckets().send().await {
            Ok(response) => {
                let buckets: Vec<String> = response.buckets()
                    .iter()
                    .filter_map(|b| b.name().map(String::from))
                    .collect();
                
                info!("Available buckets: {:?}", buckets);
                
                if buckets.contains(&self.bucket) {
                    info!("Bucket {} already exists", self.bucket);
                    return Ok(());
                }
            }
            Err(e) => {
                let error_msg = format!("Failed to list buckets: {}", e);
                error!("{}", error_msg);
                return Err(AppError::StorageError(error_msg));
            }
        }

        // Bucket doesn't exist, try to create it
        info!("Creating bucket: {}", self.bucket);
        match self.client
            .create_bucket()
            .bucket(&self.bucket)
            .send()
            .await 
        {
            Ok(_) => {
                info!("Successfully created bucket: {}", self.bucket);
                Ok(())
            }
            Err(e) => {
                let error_msg = format!("Failed to create bucket: {}", e);
                error!("{}", error_msg);
                match e {
                    SdkError::ServiceError(err) => {
                        if err.err().is_bucket_already_exists() {
                            info!("Bucket {} already exists (race condition)", self.bucket);
                            Ok(())
                        } else {
                            let error_msg = format!("MinIO service error: {}", err.err());
                            error!("{}", error_msg);
                            Err(AppError::StorageError(error_msg))
                        }
                    }
                    _ => {
                        error!("{}", error_msg);
                        Err(AppError::StorageError(error_msg))
                    }
                }
            }
        }
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
                        let error_msg = format!("MinIO service error: {}", err.err());
                        error!("{}", error_msg);
                        AppError::StorageError(error_msg)
                    }
                    _ => {
                        let error_msg = format!("Failed to list buckets: {}", e);
                        error!("{}", error_msg);
                        AppError::StorageError(error_msg)
                    }
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
        println!("DEBUG: ===== Starting storage service upload =====");
        println!("DEBUG: File path: {:?}, Key: {}", file_path, key);
        debug!("Starting file upload process");
        debug!("File path: {:?}, Key: {}", file_path, key);
        
        // Ensure bucket exists before attempting upload
        println!("DEBUG: Ensuring bucket exists");
        debug!("Ensuring bucket exists");
        self.ensure_bucket_exists().await?;
        println!("DEBUG: Bucket check completed");
        debug!("Bucket check completed");

        // Check if file exists
        if !file_path.exists() {
            let error_msg = format!("File does not exist: {:?}", file_path);
            println!("DEBUG: {}", error_msg);
            error!("{}", error_msg);
            return Err(AppError::FileError(error_msg));
        }
        println!("DEBUG: File exists at path");
        debug!("File exists at path");

        // Create byte stream directly from file
        let body = match ByteStream::from_path(file_path).await {
            Ok(stream) => {
                println!("DEBUG: Successfully created ByteStream from file");
                stream
            },
            Err(e) => {
                let error_msg = format!("Failed to create ByteStream from file: {}", e);
                println!("DEBUG: {}", error_msg);
                error!("{}", error_msg);
                return Err(AppError::FileError(error_msg));
            }
        };

        println!("DEBUG: Attempting to upload to MinIO bucket: {} with key: {}", self.bucket, key);
        info!("Attempting to upload to MinIO bucket: {} with key: {}", self.bucket, key);
        
        // Upload to MinIO
        match self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body)
            .send()
            .await 
        {
            Ok(_) => {
                println!("DEBUG: Successfully uploaded file to MinIO");
                info!("Successfully uploaded file to MinIO");
                Ok(())
            }
            Err(e) => {
                println!("DEBUG: Failed to upload file to MinIO: {}", e);
                error!("Failed to upload file to MinIO: {}", e);
                match e {
                    SdkError::ServiceError(err) => {
                        let error_msg = format!("MinIO service error: {}", err.err());
                        println!("DEBUG: {}", error_msg);
                        error!("{}", error_msg);
                        Err(AppError::StorageError(error_msg))
                    }
                    _ => {
                        let error_msg = format!("Failed to upload file: {}", e);
                        println!("DEBUG: {}", error_msg);
                        error!("{}", error_msg);
                        Err(AppError::StorageError(error_msg))
                    }
                }
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
