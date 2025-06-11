use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod db;
mod error;
mod handlers;
mod models;
mod storage;
mod services;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = config::Config::from_env().expect("Failed to load configuration");

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
    let auth_service = web::Data::new(services::AuthService::new(
        pool.clone(),
        config.jwt_secret.clone(),
    ));
    let storage_service = web::Data::new(services::StorageService::new(&config.storage_path));
    let ai_service = web::Data::new(services::AIService::new(config.openrouter_api_key.clone()));

    // Start HTTP server
    let config_clone = config.clone();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .expose_headers(["content-type", "content-length"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(auth_service.clone())
            .app_data(storage_service.clone())
            .app_data(ai_service.clone())
            .app_data(web::Data::new(config_clone.clone()))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .route("/register", web::post().to(routes::auth::register))
                            .route("/login", web::post().to(routes::auth::login))
                    )
                    .service(
                        web::scope("/upload")
                            .route("", web::post().to(handlers::upload::upload_file))
                            .route("/{file_id}", web::get().to(handlers::upload::get_file))
                    )
                    .service(
                        web::scope("/chat")
                            .route("", web::post().to(routes::chat::chat))
                    )
            )
    })
    .bind(("127.0.0.1", config.server_port))?
    .run()
    .await
}
