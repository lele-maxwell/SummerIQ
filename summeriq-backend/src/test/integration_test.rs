use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use summeriq_backend::{
    config::Config,
    db,
    services::{auth::AuthService, storage::StorageService, ai::AIService},
};
use tower::ServiceExt;
use std::sync::Arc;

async fn setup_test_app() -> (
    axum::Router,
    Arc<AuthService>,
    Arc<StorageService>,
    Arc<AIService>,
) {
    let config = Config::from_env().expect("Failed to load configuration");
    let pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let auth_service = Arc::new(AuthService::new(pool.clone(), config.jwt_secret));
    let storage_service = Arc::new(StorageService::new(
        aws_sdk_s3::Client::new(&aws_config::from_env().load().await),
        "test-bucket".to_string(),
    ));
    let ai_service = Arc::new(AIService::new(config.openrouter_api_key));

    let app = summeriq_backend::create_app(
        pool,
        auth_service.clone(),
        storage_service.clone(),
        ai_service.clone(),
    );

    (app, auth_service, storage_service, ai_service)
}

#[tokio::test]
async fn test_register_endpoint() {
    let (app, _, _, _) = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/auth/register")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&serde_json::json!({
                        "name": "Test User",
                        "email": "test@example.com",
                        "password": "password123"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_upload_endpoint() {
    let (app, auth_service, _, _) = setup_test_app().await;

    // First register and login to get a token
    let user = summeriq_backend::models::user::CreateUser {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let (_, token) = auth_service.register(user).await.unwrap();

    // Create a test zip file
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    zip.start_file("test.rs", zip::write::FileOptions::default())
        .unwrap();
    zip.write_all(b"fn main() { println!(\"Hello, world!\"); }")
        .unwrap();
    let zip_data = zip.finish().unwrap().into_inner().unwrap();

    // Test upload endpoint
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/upload")
                .method("POST")
                .header("Authorization", format!("Bearer {}", token))
                .header(
                    "content-type",
                    "multipart/form-data; boundary=----WebKitFormBoundary",
                )
                .body(Body::from(zip_data))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
