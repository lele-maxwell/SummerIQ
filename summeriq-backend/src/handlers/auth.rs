use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use tracing::{info, error};
use validator::Validate;


use crate::error::AppError;
use crate::models::user::{CreateUser, LoginUser};
use crate::services::AuthService;

pub async fn register(
    auth_service: web::Data<AuthService>,
    user_data: web::Json<CreateUser>,
) -> Result<impl Responder, AppError> {
    // Validate user data
    if let Err(e) = user_data.validate() {
        return Err(AppError::BadRequest(format!("Validation error: {}", e)));
    }
    // Try to register the user, handle duplicate email
    let user = match auth_service.register(user_data.into_inner()).await {
        Ok(user) => user,
        Err(AppError::InternalServerError(ref msg)) if msg.contains("users_email_key") || msg.contains("duplicate key value") => {
            return Err(AppError::BadRequest("Email already exists".to_string()));
        },
        Err(e) => return Err(e),
    };
    // Generate token for the new user
    let token = auth_service.generate_token(&user)?;
    Ok(HttpResponse::Created().json(json!({
        "message": "User registered successfully",
        "user": user,
        "token": token
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