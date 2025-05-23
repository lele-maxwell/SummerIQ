use aws_sdk_s3::{Client, types::ByteStream};
use crate::error::AppError;
use std::path::Path;
use tokio::fs::File;
use uuid::Uuid;

pub struct StorageService {
    client: Client,
    bucket: String,
}

impl StorageService {
    pub fn new(client: Client, bucket: String) -> Self {
        Self { client, bucket }
    }

    pub async fn upload_file(&self, file_path: &Path, user_id: Uuid) -> Result<String, AppError> {
        let file = File::open(file_path).await
            .map_err(|e| AppError::StorageError(format!("Failed to open file: {}", e)))?;

        let key = format!("{}/{}.zip", user_id, Uuid::new_v4());
        let body = ByteStream::from_file(file).await
            .map_err(|e| AppError::StorageError(format!("Failed to create byte stream: {}", e)))?;

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(body)
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Failed to upload file: {}", e)))?;

        Ok(key)
    }

    pub async fn get_file_url(&self, key: &str) -> Result<String, AppError> {
        let presigned = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(aws_sdk_s3::presigning::config::PresigningConfig::builder()
                .expires_in(std::time::Duration::from_secs(3600))
                .build()
                .map_err(|e| AppError::StorageError(format!("Failed to build presigning config: {}", e)))?)
            .await
            .map_err(|e| AppError::StorageError(format!("Failed to generate presigned URL: {}", e)))?;

        Ok(presigned.uri().to_string())
    }

    pub async fn delete_file(&self, key: &str) -> Result<(), AppError> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Failed to delete file: {}", e)))?;

        Ok(())
    }
}
