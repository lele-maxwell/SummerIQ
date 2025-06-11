use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::{info, error};

use crate::error::AppError;
use crate::services::AIService;

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub response: String,
}

pub async fn chat(
    ai_service: web::Data<AIService>,
    request: web::Json<ChatRequest>,
) -> Result<impl Responder, AppError> {
    let response = ai_service.analyze_text(&request.message)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    info!("Chat response generated successfully");
    Ok(HttpResponse::Ok().json(ChatResponse { response }))
} 