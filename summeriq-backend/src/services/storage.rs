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
        debug!("Checking if bucket '{}' exists", self.bucket);
        
        match self.client.list_buckets().send().await {
            Ok(response) => {
                let buckets: Vec<String> = response.buckets()
                    .iter()
                    .filter_map(|b| b.name().map(String::from))
                    .collect();
                
                debug!("Available buckets: {:?}", buckets);
                
                if buckets.contains(&self.bucket) {
                    debug!("Bucket '{}' already exists", self.bucket);
                    return Ok(());
                }
            }
            Err(e) => {
                let sdk_error = match &e {
                    SdkError::ServiceError(err) => format!("S3 Service Error: {}", err.err()),
                    _ => e.to_string(),
                };
                error!("Failed to list buckets while checking existence of bucket '{}': {}. SDK Error: {}", self.bucket, e, sdk_error);
                return Err(AppError::StorageError(format!("Failed to list buckets: {}. SDK Error: {}", e, sdk_error)));
            }
        }

        debug!("Bucket '{}' does not exist, attempting to create it.", self.bucket);
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
                let sdk_error = match &e {
                    SdkError::ServiceError(err) => format!("S3 Service Error: {}", err.err()),
                    _ => e.to_string(),
                };
                error!("Failed to create bucket '{}': {}. SDK Error: {}", self.bucket, e, sdk_error);
                match e {
                    SdkError::ServiceError(err) => {
                        if err.err().is_bucket_already_exists() {
                            info!("Bucket '{}' already exists (race condition during creation)", self.bucket);
                            Ok(())
                        } else {
                            Err(AppError::StorageError(format!("Failed to create bucket '{}': MinIO service error: {}", self.bucket, err.err())))
                        }
                    }
                    _ => {
                        Err(AppError::StorageError(format!("Failed to create bucket '{}': {}. SDK Error: {}", self.bucket, e, sdk_error)))
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
                let sdk_error = match &e {
                    SdkError::ServiceError(err) => format!("S3 Service Error: {}", err.err()),
                    _ => e.to_string(),
                };
                error!("Failed to list MinIO buckets: {}. SDK Error: {}", e, sdk_error);
                AppError::StorageError(format!("Failed to list MinIO buckets: {}. SDK Error: {}", e, sdk_error))
            })?;

        let buckets: Vec<String> = response.buckets()
            .iter()
            .filter_map(|b| b.name().map(String::from))
            .collect();

        debug!("Found buckets: {:?}", buckets);
        Ok(buckets)
    }

    pub async fn upload_file(&self, file_path: &Path, key: &str) -> Result<(), AppError> {
        debug!("===== Starting storage service upload =====");
        debug!("File path: {:?}, Key: {}", file_path, key);
        
        debug!("Ensuring bucket '{}' exists", self.bucket);
        self.ensure_bucket_exists().await?;
        debug!("Bucket '{}' check completed", self.bucket);

        if !file_path.exists() {
            let error_msg = format!("File does not exist: {:?}", file_path);
            error!("{}", error_msg);
            return Err(AppError::FileError(error_msg));
        }
        debug!("File exists at path: {:?}", file_path);

        let body = match ByteStream::from_path(file_path).await {
            Ok(stream) => {
                debug!("Successfully created ByteStream from file: {:?}", file_path);
                stream
            },
            Err(e) => {
                let error_msg = format!("Failed to create ByteStream from file '{:?}': {}", file_path, e);
                error!("{}", error_msg);
                return Err(AppError::FileError(error_msg));
            }
        };

        info!("Attempting to upload to MinIO bucket: '{}' with key: '{}'", self.bucket, key);
        
        match self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body)
            .send()
            .await 
        {
            Ok(_) => {
                info!("Successfully uploaded file '{:?}' to MinIO bucket '{}' with key '{}'", file_path, self.bucket, key);
                Ok(())
            }
            Err(e) => {
                let sdk_error = match &e {
                    SdkError::ServiceError(err) => format!("S3 Service Error: {}", err.err()),
                    _ => e.to_string(),
                };
                error!("Failed to upload file '{:?}' to MinIO bucket '{}' with key '{}': {}. SDK Error: {}", file_path, self.bucket, key, e, sdk_error);
                Err(AppError::StorageError(format!("Failed to upload file to bucket '{}' with key '{}': {}. SDK Error: {}", self.bucket, key, e, sdk_error)))
            }
        }
    }

    pub async fn get_file(&self, key: &str) -> Result<Vec<u8>, anyhow::Error> {
        debug!("Attempting to get file with key '{}' from bucket '{}'", key, self.bucket);
        let response = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        let bytes = response.body.collect().await?;
        debug!("Successfully retrieved file with key '{}' from bucket '{}'", key, self.bucket);
        Ok(bytes.to_vec())
    }

    pub async fn delete_file(&self, key: &str) -> Result<(), anyhow::Error> {
        debug!("Attempting to delete file with key '{}' from bucket '{}'", key, self.bucket);
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;
        info!("Successfully deleted file with key '{}' from bucket '{}'", key, self.bucket);
        Ok(())
    }
}
