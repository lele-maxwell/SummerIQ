use axum::{
    body::{Body, Bytes, to_bytes},
    http::{Request, StatusCode, header::CONTENT_TYPE},
};
use reqwest::multipart;
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
        config.minio_bucket_name.clone(),
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
    let user_email = "upload_test_user@example.com";
    let create_user_payload = summeriq_backend::models::user::CreateUser {
        name: "Upload Test User".to_string(),
        email: user_email.to_string(),
        password: "password123".to_string(),
    };

    // It's good practice to handle potential errors, even in tests.
    let registered_user = auth_service.register(create_user_payload).await;
    assert!(registered_user.is_ok(), "User registration failed: {:?}", registered_user.err());
    let (user_id, token) = registered_user.unwrap();

    // Create a tiny valid zip file in memory
    let mut zip_buffer = Vec::new();
    let mut zip_writer = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_buffer));
    let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    zip_writer.start_file("test_file.txt", options).expect("Failed to start zip file");
    zip_writer.write_all(b"This is a test file inside a zip.").expect("Failed to write to zip");
    zip_writer.finish().expect("Failed to finish zip");

    let file_name = "test_upload.zip";
    let part = multipart::Part::bytes(zip_buffer)
        .file_name(file_name.to_string())
        .mime_str("application/zip")
        .unwrap();

    let form = multipart::Form::new().part("file", part);
    let boundary = form.boundary().to_string();
    let content_type_header = format!("multipart/form-data; boundary={}", boundary);
    
    // Convert reqwest::Body to axum::Body
    // This is a bit convoluted because reqwest::Body is a stream.
    let stream = form.stream();
    let body_bytes = hyper::body::to_bytes(stream).await.expect("Failed to read multipart stream");
    let body = Body::from(body_bytes);

    // Test upload endpoint
    let request = Request::builder()
        .uri("/api/upload")
        .method("POST")
        .header("Authorization", format!("Bearer {}", token))
        .header(CONTENT_TYPE, content_type_header)
        .body(body)
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK, "Upload request failed. Response: {:?}", response);

    let response_body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&response_body_bytes).expect("Failed to parse JSON response");

    assert_eq!(response_json["fileName"], file_name, "fileName mismatch");
    assert!(response_json["key"].is_string(), "key should be a string");
    assert!(response_json["key"].as_str().unwrap().contains(file_name), "key should contain filename");
    // Ideally, also check if key contains user_id, e.g., response_json["key"].as_str().unwrap().starts_with(&user_id.to_string())
    // However, user_id from register is a Uuid, and the key uses user_id.to_string() from auth_user.0 which is an i32.
    // This implies a difference in how user ID is handled (AuthUser.0 vs User.id).
    // For now, we'll check that the key seems reasonable.
    assert!(response_json["key"].as_str().unwrap().contains(&user_id.to_string()), "key should contain user_id");


    assert_eq!(response_json["contentType"], "application/zip", "contentType mismatch");
}
