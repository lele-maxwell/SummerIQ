use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{Response, IntoResponse},
};
use std::sync::Arc;
use tracing::{error, debug};
use serde_json::json;

use crate::error::AppError;
use crate::services::auth::AuthService;
use crate::services::auth::AuthUser;
use crate::services::StorageService;
use crate::services::AIService;

pub async fn auth_middleware(
    State((auth_service, _storage_service, _ai_service)): State<(Arc<AuthService>, Arc<StorageService>, Arc<AIService>)>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, AppError> {
    println!("DEBUG: ===== Request reached auth middleware =====");
    println!("DEBUG: Request path: {}", request.uri().path());
    println!("DEBUG: Request method: {}", request.method());

    // Skip authentication for login and register endpoints
    if request.uri().path() == "/api/auth/login" || request.uri().path() == "/api/auth/register" {
        println!("DEBUG: Skipping authentication for login/register endpoint");
        return Ok(next.run(request).await);
    }

    // Get the authorization header
    let auth_header = request.headers()
        .get("authorization")
        .and_then(|value| value.to_str().ok());

    match auth_header {
        Some(header) => {
            println!("DEBUG: Found authorization header: {}", header);
            if !header.starts_with("Bearer ") {
                println!("DEBUG: Invalid authorization header format");
                return Ok(StatusCode::UNAUTHORIZED.into_response());
            }

            let token = &header[7..];
            println!("DEBUG: Extracted token: {}", token);

            // Verify the token
            match auth_service.verify_token(token) {
                Ok(claims) => {
                    println!("DEBUG: Token verified successfully for user: {:?}", claims.sub);
                    // Add user to request extensions
                    let mut request = request;
                    request.extensions_mut().insert(AuthUser(claims.sub));
                    Ok(next.run(request).await)
                }
                Err(e) => {
                    println!("DEBUG: Token verification failed: {:?}", e);
                    Ok(StatusCode::UNAUTHORIZED.into_response())
                }
            }
        }
        None => {
            println!("DEBUG: No authorization header found");
            Ok(StatusCode::UNAUTHORIZED.into_response())
        }
    }
} 