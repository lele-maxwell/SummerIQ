use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::filter::EnvFilter;
use std::env;

mod config;
mod db;
mod error;
mod models;
mod storage;
mod services;
mod routes;
mod handlers;

use routes::auth;
use routes::upload;
use routes::analysis;

use config::Config;
use services::{StorageService, AnalysisService, AIService, AuthService};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env();

    // Create storage directory if it doesn't exist
    std::fs::create_dir_all(&config.storage_path)
        .expect("Failed to create storage directory");

    // Initialize database connection
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    // Initialize services
    let storage_service = StorageService::new(config.storage_path.clone());
    let storage_service_data = web::Data::new(storage_service.clone());
    
    let ai_service = AIService::new(config.groq_api_key.clone());
    let ai_service_data = web::Data::new(ai_service.clone());
    
    let analysis_service = web::Data::new(AnalysisService::new(
        config.groq_api_key.clone(),
        storage_service.clone(),
        ai_service.clone(),
    ));
    let auth_service = web::Data::new(AuthService::new(pool.clone(), config.jwt_secret.clone()));

    // Start HTTP server
    let config_clone = config.clone();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(config_clone.clone()))
            .app_data(web::Data::new(pool.clone()))
            .app_data(storage_service_data.clone())
            .app_data(analysis_service.clone())
            .app_data(ai_service_data.clone())
            .app_data(auth_service.clone())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .route("/register", web::post().to(handlers::auth::register))
                            .route("/login", web::post().to(handlers::auth::login))
                    )
                    .service(
                        web::scope("/upload")
                            .route("", web::post().to(handlers::upload::upload_file))
                            .route("/{file_id}", web::get().to(upload::get_file))
                            .route("/content/{path:.*}", web::get().to(upload::get_file_content))
                    )
                    .service(
                        web::scope("/analysis")
                            .route("/file/{path:.*}", web::get().to(routes::analysis::analyze_file))
                    )
            )
    })
    .bind(("127.0.0.1", config.server_port))?
    .run()
    .await
}
