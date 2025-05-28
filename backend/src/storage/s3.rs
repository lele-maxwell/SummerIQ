use aws_sdk_s3::{
    config::Region,
    types::{BucketLocationConstraint, CreateBucketConfiguration},
    Client,
};
use aws_sdk_s3::primitives::ByteStream;
use std::error::Error;
use tracing::{info, error};

pub struct S3Client {
    client: Client,
}

impl S3Client {
    pub async fn new(endpoint: &str, access_key: &str, secret_key: &str) -> Result<Self, Box<dyn Error>> {
        info!("Initializing S3 client with endpoint: {}", endpoint);
        
        let config = aws_sdk_s3::config::Builder::new()
            .endpoint_url(endpoint)
            .region(Region::new("us-east-1"))
            .credentials_provider(aws_sdk_s3::config::Credentials::new(
                access_key,
                secret_key,
                None,
                None,
                "StaticProvider",
            ))
            .force_path_style(true)
            .build();

        let client = Client::from_conf(config);
        
        // Test the connection
        match client.list_buckets().send().await {
            Ok(output) => {
                let buckets: Vec<String> = output
                    .buckets()
                    .unwrap_or_default()
                    .iter()
                    .filter_map(|b| b.name().map(String::from))
                    .collect();
                info!("Successfully connected to S3. Available buckets: {:?}", buckets);
            }
            Err(e) => {
                error!("Failed to connect to S3: {}", e);
                return Err(Box::new(e));
            }
        }

        Ok(Self { client })
    }
}

pub async fn create_bucket(&self, bucket_name: &str) -> Result<(), Box<dyn Error>> {
    info!("Attempting to create bucket: {}", bucket_name);
    // Replace forward slashes with hyphens for bucket name
    let sanitized_bucket_name = bucket_name.replace('/', "-");
    info!("Sanitized bucket name: {}", sanitized_bucket_name);
    
    let config = CreateBucketConfiguration::builder()
        .location_constraint(BucketLocationConstraint::UsEast1)
        .build();

    info!("Checking if bucket exists: {}", sanitized_bucket_name);
    match self.client.head_bucket().bucket(&sanitized_bucket_name).send().await {
        Ok(_) => {
            info!("Bucket {} already exists", sanitized_bucket_name);
            return Ok(());
        }
        Err(e) => {
            info!("Bucket {} does not exist, will create it: {}", sanitized_bucket_name, e);
        }
    }

    info!("Creating bucket with configuration: {:?}", config);
    match self
        .client
        .create_bucket()
        .bucket(&sanitized_bucket_name)
        .create_bucket_configuration(config)
        .send()
        .await
    {
        Ok(_) => {
            info!("Successfully created bucket: {}", sanitized_bucket_name);
            Ok(())
        }
        Err(e) => {
            error!("Failed to create bucket {}: {}", sanitized_bucket_name, e);
            error!("Error details: {:?}", e);
            Err(Box::new(e))
        }
    }
}

pub async fn upload_file(
    &self,
    bucket_name: &str,
    key: &str,
    body: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    info!("Attempting to upload file to bucket: {}, key: {}", bucket_name, key);
    // Replace forward slashes with hyphens for bucket name
    let sanitized_bucket_name = bucket_name.replace('/', "-");
    info!("Sanitized bucket name: {}", sanitized_bucket_name);
    
    let body = ByteStream::from(body);
    info!("Created ByteStream from file data, size: {} bytes", body.len());

    match self
        .client
        .put_object()
        .bucket(&sanitized_bucket_name)
        .key(key)
        .body(body)
        .send()
        .await
    {
        Ok(_) => {
            info!("Successfully uploaded file to {}/{}", sanitized_bucket_name, key);
            Ok(())
        }
        Err(e) => {
            error!("Failed to upload file to {}/{}: {}", sanitized_bucket_name, key, e);
            Err(Box::new(e))
        }
    }
} 