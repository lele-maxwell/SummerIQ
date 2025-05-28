use crate::config::Config;
use crate::storage::s3::S3Client;
use std::sync::Arc;
use tracing::{info, error, debug};

pub struct StorageService {
    s3_client: Arc<S3Client>,
}

impl StorageService {
    pub async fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Initializing storage service");
        let s3_client = S3Client::new(
            &config.s3_endpoint,
            &config.s3_access_key,
            &config.s3_secret_key,
        ).await?;
        
        // Ensure the default bucket exists
        let bucket_name = "uploaded-folder";
        debug!("Ensuring default bucket exists: {}", bucket_name);
        s3_client.create_bucket(bucket_name).await.map_err(|e| {
            error!("Failed to create default bucket {}: {}", bucket_name, e);
            e
        })?;
        
        Ok(Self {
            s3_client: Arc::new(s3_client),
        })
    }

    pub async fn store_file(&self, bucket_name: &str, key: &str, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Storing file in bucket: {}, key: {}", bucket_name, key);
        
        // Upload the file
        debug!("Uploading file to bucket: {}, key: {}", bucket_name, key);
        self.s3_client.upload_file(bucket_name, key, data).await.map_err(|e| {
            error!("Failed to upload file to {}/{}: {}", bucket_name, key, e);
            e
        })?;
        
        info!("Successfully stored file in {}/{}", bucket_name, key);
        Ok(())
    }
} 