use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::services::AuthService;
use chrono::Utc;
use validator;

use crate::{
    error::AppError,
    models::user::{CreateUser, LoginUser, User},
};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub fn auth_router() -> Router<(Arc<AuthService>, Arc<crate::services::StorageService>, Arc<crate::services::AIService>)> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}

async fn register(
    State((auth_service, _, _)): State<(Arc<AuthService>, Arc<crate::services::StorageService>, Arc<crate::services::AIService>)>,
    Json(credentials): Json<RegisterRequest>,
) -> Result<Json<User>, AppError> {
    // Validate email format
    if !validator::validate_email(&credentials.email) {
        return Err(AppError::ValidationError("Invalid email format".to_string()));
    }

    // Validate password length
    if credentials.password.len() < 8 {
        return Err(AppError::ValidationError("Password must be at least 8 characters long".to_string()));
    }

    let auth_user = auth_service.register(&credentials.name, &credentials.email, &credentials.password)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    Ok(Json(auth_user))
}

async fn login(
    State((auth_service, _, _)): State<(Arc<AuthService>, Arc<crate::services::StorageService>, Arc<crate::services::AIService>)>,
    Json(credentials): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    println!("DEBUG: ===== Starting login handler =====");
    println!("DEBUG: Attempting login for email: {}", credentials.email);
    
    let (user, token) = auth_service.login(&credentials.email, &credentials.password)
        .await
        .map_err(|e| {
            println!("DEBUG: Login failed: {:?}", e);
            e
        })?;
    
    println!("DEBUG: Login successful for user: {}", user.id);
    
    Ok(Json(LoginResponse { token }))
}
