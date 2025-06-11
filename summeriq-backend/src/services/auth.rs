use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Utc, Duration};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::user::User,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct AuthService {
    pool: PgPool,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(pool: PgPool, jwt_secret: String) -> Self {
        Self { pool, jwt_secret }
    }

    pub async fn register(&self, name: &str, email: &str, password: &str) -> Result<User, AppError> {
        // Check if user already exists
        let existing_user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

        if existing_user.is_some() {
            return Err(AppError::ValidationError("User already exists".to_string()));
        }

        // Hash password
        let password_hash = hash(password, DEFAULT_COST)
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        let now = Utc::now();

        // Create user
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, name, email, password_hash, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6) 
             RETURNING id, name, email, password_hash, created_at, updated_at"
        )
        .bind(Uuid::new_v4())
        .bind(name)
        .bind(email)
        .bind(&password_hash)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(user)
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<(User, String), AppError> {
        // Find user by email
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, password_hash, created_at, updated_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?
        .ok_or_else(|| AppError::AuthError("Invalid credentials".to_string()))?;

        // Verify password
        if !bcrypt::verify(password, &user.password_hash)
            .map_err(|e| AppError::InternalError(e.to_string()))? {
            return Err(AppError::AuthError("Invalid credentials".to_string()));
        }

        // Generate JWT token
        let token = self.generate_token(&user.id.to_string())?;

        Ok((user, token))
    }

    fn generate_token(&self, user_id: &str) -> Result<String, AppError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_owned(),
            exp: expiration,
        };

        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalError(e.to_string()))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| AppError::AuthError(e.to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct AuthUser(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let token = auth_header.trim_start_matches("Bearer ");
        let auth_service = parts
            .extensions
            .get::<Arc<AuthService>>()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        let claims = auth_service.verify_token(token)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
        Ok(AuthUser(claims.sub))
    }
}
