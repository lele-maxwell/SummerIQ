use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use tracing::{info, error};

use crate::error::AppError;
use crate::models::user::{CreateUser, LoginUser};
use crate::services::AuthService;

pub async fn register(
    auth_service: web::Data<AuthService>,
    user_data: web::Json<CreateUser>,
) -> Result<impl Responder, AppError> {
    let user = auth_service.register(user_data.into_inner()).await?;
    Ok(HttpResponse::Created().json(json!({
        "message": "User registered successfully",
        "user": user
    })))
}

pub async fn login(
    auth_service: web::Data<AuthService>,
    credentials: web::Json<LoginUser>,
) -> Result<impl Responder, AppError> {
    let (user, token) = auth_service.login(credentials.into_inner()).await?;
    Ok(HttpResponse::Ok().json(json!({
        "message": "Login successful",
        "user": user,
        "token": token
    })))
} 