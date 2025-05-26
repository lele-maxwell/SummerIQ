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

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
}

pub struct AuthService {
    pool: PgPool,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(pool: PgPool, jwt_secret: String) -> Self {
        Self { pool, jwt_secret }
    }

    pub async fn register(&self, email: &str, password: &str) -> Result<User, StatusCode> {
        tracing::debug!("Registering new user: {}", email);
        
        let password_hash = hash(password, DEFAULT_COST)
            .map_err(|e| {
                tracing::error!("Password hashing error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        tracing::debug!("Password hashed successfully");

        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id, email, password_hash",
            email,
            password_hash 
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error during registration: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        tracing::debug!("User registered successfully: {}", email);
        Ok(user)
    }

    pub async fn authenticate(&self, email: &str, password: &str) -> Result<String, StatusCode> {
        tracing::debug!("Attempting to authenticate user: {}", email);
        
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, password_hash FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error during authentication: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or_else(|| {
            tracing::warn!("User not found: {}", email);
            StatusCode::UNAUTHORIZED
        })?;

        tracing::debug!("User found, verifying password");

        let password_verified = bcrypt::verify(password, &user.password_hash)
            .map_err(|e| {
                tracing::error!("Password verification error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        if !password_verified {
            tracing::warn!("Password verification failed for user: {}", email);
            return Err(StatusCode::UNAUTHORIZED);
        }

        tracing::debug!("Password verified, generating token");

        let claims = Claims {
            sub: user.id,
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| {
            tracing::error!("Token generation error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        tracing::debug!("Authentication successful for user: {}", email);
        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, StatusCode> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| StatusCode::UNAUTHORIZED)
    }
}

#[derive(Debug)]
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

        let claims = auth_service.verify_token(token)?;
        Ok(AuthUser(claims.sub))
    }
}
