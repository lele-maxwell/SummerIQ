use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
    extract::State,
};
use std::sync::Arc;
use crate::services::{AuthService, StorageService, AIService};
use crate::error::AppError;
use tracing::{info, error, debug, warn};

pub async fn auth_middleware(
    State(state): State<(Arc<AuthService>, Arc<StorageService>, Arc<AIService>)>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let (auth_service, _, _) = state;
    debug!("Processing request in auth middleware");
    debug!("Request path: {}", req.uri().path());
    debug!("Request method: {}", req.method());
    
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            error!("Missing authorization header");
            AppError::AuthError("Missing authorization header".into())
        })?;

    debug!("Found authorization header: {}", auth_header);

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| {
            error!("Invalid token format");
            AppError::AuthError("Invalid token format".into())
        })?;

    debug!("Extracted token, verifying...");

    let claims = auth_service.verify_token(token).map_err(|e| {
        error!("Token verification failed: {}", e);
        AppError::AuthError(format!("Invalid token: {}", e))
    })?;

    debug!("Token verified successfully for user: {}", claims.sub);

    // Add user_id to request extensions
    let mut req = req;
    req.extensions_mut().insert(claims.sub);

    debug!("Proceeding to next middleware/handler");
    Ok(next.run(req).await)
} 