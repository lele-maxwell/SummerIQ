use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenv::dotenv;
use std::env;

mod auth;
mod auth_middleware;
mod config;
mod error;
mod models;
mod routes;
mod services;
mod storage;

use crate::{
    auth::AuthService,
    config::Config,
    services::{StorageService, UploadService},
    routes::upload::upload_router,
};

#[derive(Clone)]
pub struct AppState {
    auth_service: Arc<AuthService>,
    storage_service: Arc<StorageService>,
    upload_service: Arc<UploadService>,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenv().ok();
    println!("Current directory: {:?}", std::env::current_dir().unwrap());
    println!(
        "Loaded .env file from: {:?}",
        std::env::var("DOTENV_PATH").unwrap_or_else(|_| ".env".to_string())
    );

    // Initialize services
    let config = Config::from_env().expect("Failed to load config");
    let auth_service = Arc::new(AuthService::new(&config));
    let storage_service = Arc::new(StorageService::new(&config).await.expect("Failed to initialize storage service"));
    let upload_service = Arc::new(UploadService::new(&config));

    let state = AppState {
        auth_service,
        storage_service,
        upload_service,
    };

    // Build router
    let app = Router::new()
        .merge(upload_router())
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(state));

    // Start server
    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
} 