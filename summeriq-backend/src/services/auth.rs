use actix_web::web;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{info, error};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::user::{CreateUser, LoginUser, User};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct AuthService {
    pub pool: PgPool,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(pool: PgPool, jwt_secret: String) -> Self {
        Self {
            pool,
            jwt_secret,
        }
    }

    pub async fn register(&self, user_data: CreateUser) -> Result<User, AppError> {
        let password_hash = hash(user_data.password.as_bytes(), DEFAULT_COST)
            .map_err(|e| AppError::AuthenticationError(e.to_string()))?;

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, password_hash, full_name)
            VALUES ($1, $2, $3)
            RETURNING id, email, password_hash, full_name, created_at, updated_at
            "#,
            user_data.email,
            password_hash,
            user_data.full_name
        )
        .fetch_one(&self.pool)
        .await?;

        info!("User registered successfully: {}", user.email);
        Ok(user)
    }

    pub async fn login(&self, credentials: LoginUser) -> Result<(User, String), AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, full_name, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            credentials.email
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::AuthenticationError("Invalid credentials".to_string()))?;

        if !verify(credentials.password, &user.password_hash)
            .map_err(|e| AppError::AuthenticationError(e.to_string()))?
        {
            return Err(AppError::AuthenticationError("Invalid credentials".to_string()));
        }

        let token = self.generate_token(&user)?;
        info!("User logged in successfully: {}", user.email);
        Ok((user, token))
    }

    pub fn generate_token(&self, user: &User) -> Result<String, AppError> {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id.to_string(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::AuthenticationError(e.to_string()))
    }

    pub fn verify_token(&self, token: &str) -> Result<Uuid, AppError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::AuthenticationError(e.to_string()))?;

        Uuid::parse_str(&token_data.claims.sub)
            .map_err(|e| AppError::AuthenticationError(e.to_string()))
    }
} 