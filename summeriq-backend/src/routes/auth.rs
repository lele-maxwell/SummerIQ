use axum::{
    extract::State,
    routing::{post},
    Router,
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::user::{CreateUser, LoginUser, User},
    services::auth::generate_token,
};

pub fn auth_router() -> Router<PgPool> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    let hashed_password = hash(payload.password.as_bytes(), DEFAULT_COST)
        .map_err(|_| AppError::InternalError)?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, email, hashed_password)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        payload.name,
        payload.email,
        hashed_password
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(user))
}

async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginUser>,
) -> Result<Json<String>, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users WHERE email = $1
        "#,
        payload.email
    )
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::AuthError("Invalid credentials".into()))?;

    let valid = verify(&payload.password, &user.hashed_password)
        .map_err(|_| AppError::InternalError)?;

    if !valid {
        return Err(AppError::AuthError("Invalid credentials".into()));
    }

    let token = generate_token(user.id)?;
    Ok(Json(token))
}
