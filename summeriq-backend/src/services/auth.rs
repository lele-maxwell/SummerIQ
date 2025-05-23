use crate::{
    error::AppError,
    models::user::{User, CreateUser, LoginUser},
};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Claims};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: Uuid,
    exp: usize,
}

pub struct AuthService {
    pool: PgPool,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(pool: PgPool, jwt_secret: String) -> Self {
        Self { pool, jwt_secret }
    }

    pub async fn register(&self, user: CreateUser) -> Result<User, AppError> {
        let hashed_password = hash(user.password.as_bytes(), DEFAULT_COST)
            .map_err(|_| AppError::InternalError)?;

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, email, hashed_password)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            user.name,
            user.email,
            hashed_password
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn login(&self, credentials: LoginUser) -> Result<(User, String), AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE email = $1
            "#,
            credentials.email
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::AuthError("Invalid credentials".into()))?;

        let valid = verify(&credentials.password, &user.hashed_password)
            .map_err(|_| AppError::InternalError)?;

        if !valid {
            return Err(AppError::AuthError("Invalid credentials".into()));
        }

        let token = self.generate_token(user.id)?;
        Ok((user, token))
    }

    pub fn generate_token(&self, user_id: Uuid) -> Result<String, AppError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(7))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = JwtClaims {
            sub: user_id,
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|_| AppError::InternalError)
    }

    pub fn verify_token(&self, token: &str) -> Result<Uuid, AppError> {
        let claims = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::AuthError("Invalid token".into()))?
        .claims;

        Ok(claims.sub)
    }
}
