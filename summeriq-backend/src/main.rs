use axum::{
    routing::Router,
};
use std::sync::Arc;
use tower_http::cors::{CorsLayer, AllowOrigin, Any};
use tower_http::trace::TraceLayer;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::http::Method;
use axum::http::header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE, HeaderName};
use std::time::Duration;
use axum::http::HeaderValue;
use tracing::info;
use dotenv::dotenv;
use sqlx::PgPool;
use axum::Extension;
use std::path::PathBuf;

mod config;
mod error;
mod models;
mod routes;
mod services;
mod auth_middleware;

use routes::auth::auth_router;
use routes::upload::upload_router;

use crate::services::{AuthService, StorageService, AIService};
use crate::routes::{auth, upload, chat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Print current directory
    println!("Current directory: {:?}", std::env::current_dir()?);
    
    // Load .env file
    match dotenv() {
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

    // Initialize upload directory
    let upload_dir = std::env::current_dir()?.join("uploads");
    tracing::info!("Using upload directory: {:?}", upload_dir);

    // Initialize services
    let auth_service = Arc::new(services::auth::AuthService::new(pool.clone(), config.jwt_secret));
    let storage_service = Arc::new(services::storage::StorageService::new(upload_dir));
    let ai_service = Arc::new(services::ai::AIService::new(config.openrouter_api_key));

    // Build the router
    let state = (Arc::clone(&auth_service), Arc::clone(&storage_service), Arc::clone(&ai_service));
    
    let cors = CorsLayer::new()
        .allow_origin(vec![
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:8080".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:8080".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([
            AUTHORIZATION,
            ACCEPT,
            CONTENT_TYPE,
            HeaderName::from_static("x-requested-with"),
            HeaderName::from_static("origin"),
            HeaderName::from_static("access-control-request-method"),
            HeaderName::from_static("access-control-request-headers"),
        ])
        .expose_headers([
            HeaderName::from_static("content-length"),
            HeaderName::from_static("content-type"),
        ])
        .max_age(Duration::from_secs(3600))
        .allow_credentials(true);

    let app = Router::new()
        .nest("/api", Router::new()
            .merge(auth::auth_router())
            .merge(upload::upload_router(
                auth_service.clone(),
                storage_service.clone(),
                ai_service.clone(),
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
