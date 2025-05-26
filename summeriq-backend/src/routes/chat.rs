use axum::{
    extract::State,
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::services::{AuthService, AIService};
use crate::services::auth::AuthUser;

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub response: String,
}

pub fn chat_router() -> Router<(Arc<AuthService>, Arc<crate::services::StorageService>, Arc<AIService>)> {
    Router::new()
        .route("/chat", post(chat))
}

async fn chat(
    State((_, _, ai_service)): State<(Arc<AuthService>, Arc<crate::services::StorageService>, Arc<AIService>)>,
    _auth_user: AuthUser,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, axum::http::StatusCode> {
    let messages = vec![
        crate::services::ai::ChatMessage {
            role: "user".to_string(),
            content: request.message,
        },
    ];

    let response = ai_service.chat(messages).await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ChatResponse { response }))
}
