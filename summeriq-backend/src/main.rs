mod config;
mod db;
mod error;
mod middleware;
mod models;
mod routes;
mod services;

use axum::{
    routing::{get, post},
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    config::Config,
    services::{auth::AuthService, storage::StorageService, ai::AIService},
    logging::setup_logging,
};

#[tokio::main]
async fn main() {
    // Initialize logging
    setup_logging();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");

    // Initialize database connection
    let pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to connect to database");

    // Initialize MinIO client
    let s3_config = aws_config::from_env()
        .endpoint_url(&config.minio_endpoint)
        .credentials_provider(aws_sdk_s3::config::Credentials::new(
            &config.minio_access_key,
            &config.minio_secret_key,
            None,
            None,
            "minio",
        ))
        .load()
        .await;

    let s3_client = aws_sdk_s3::Client::new(&s3_config);

    // Initialize services
    let auth_service = Arc::new(AuthService::new(pool.clone(), config.jwt_secret));
    let storage_service = Arc::new(StorageService::new(s3_client, "uploaded-folders".to_string()));
    let ai_service = Arc::new(AIService::new(config.openrouter_api_key));

    // Build our application with routes
    let app = Router::new()
        .merge(routes::auth::auth_router())
        .merge(routes::upload::upload_router())
        .merge(routes::chat::chat_router())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state((pool, storage_service, ai_service));

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_port));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
