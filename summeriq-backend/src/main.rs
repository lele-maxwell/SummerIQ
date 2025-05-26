use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use std::sync::Arc;
use tower_http::cors::{CorsLayer, AllowOrigin};
use tower_http::trace::TraceLayer;
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client as S3Client;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::http::Method;
use axum::http::header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE, HeaderName};
use axum::Extension;
use std::time::Duration;
use axum::http::HeaderValue;

mod config;
mod error;
mod models;
mod routes;
mod services;
mod auth_middleware;

use routes::auth::auth_router;
use routes::upload::upload_router;
use auth_middleware::auth_middleware;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Print current directory
    println!("Current directory: {:?}", std::env::current_dir()?);
    
    // Load .env file
    match dotenv::dotenv() {
        Ok(path) => println!("Loaded .env file from: {:?}", path),
        Err(e) => println!("Error loading .env file: {}", e),
    }
    
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = config::Config::from_env()?;

    // Initialize database connection
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    // Initialize MinIO client
    let shared_config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url(config.minio_endpoint.trim_end_matches('/').to_string())
        .region("us-east-1")
        .credentials_provider(aws_sdk_s3::config::Credentials::new(
            &config.minio_access_key,
            &config.minio_secret_key,
            None,
            None,
            "minio",
        ))
        .load()
        .await;

    let s3_config = aws_sdk_s3::config::Builder::from(&shared_config)
        .force_path_style(true)
        .build();

    tracing::info!("Initializing MinIO client with endpoint: {}", config.minio_endpoint);
    let s3_client = S3Client::from_conf(s3_config);

    // Test MinIO connection
    match s3_client.list_buckets().send().await {
        Ok(buckets) => {
            tracing::info!(
                "Successfully connected to MinIO. Available buckets: {:?}",
                buckets.buckets().iter()
                    .filter_map(|b| b.name().map(String::from))
                    .collect::<Vec<String>>()
            );
        }
        Err(e) => {
            tracing::error!("Failed to connect to MinIO: {}", e);
        }
    }

    // Initialize services
    let auth_service = Arc::new(services::auth::AuthService::new(pool.clone(), config.jwt_secret));
    let storage_service = Arc::new(services::storage::StorageService::new(s3_client, "uploaded-folders".to_string()));
    let ai_service = Arc::new(services::ai::AIService::new(config.openrouter_api_key));

    // Configure CORS
    let allowed_origins = [
        HeaderValue::from_static("http://localhost:8080"),
        HeaderValue::from_static("http://localhost:3000"),
    ];
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed_origins))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([
            AUTHORIZATION,
            ACCEPT,
            CONTENT_TYPE,
            HeaderName::from_static("x-requested-with"),
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));

    // Build the router
    let state = (Arc::clone(&auth_service), Arc::clone(&storage_service), Arc::clone(&ai_service));
    let app = Router::new()
        .nest("/api", Router::new()
            .merge(auth_router())
            .merge(upload_router(
                Arc::clone(&auth_service),
                Arc::clone(&storage_service),
                Arc::clone(&ai_service),
            ))
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr = format!("0.0.0.0:{}", config.server_port);
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
