use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::{info, error};

use crate::error::AppError;
use crate::services::AIService;

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub project_name: Option<String>,
    pub selected_file_name: Option<String>,
    pub selected_file_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub response: String,
}

pub async fn chat(
    ai_service: web::Data<AIService>,
    request: web::Json<ChatRequest>,
) -> Result<impl Responder, AppError> {
    // Build context-aware prompt
    let mut prompt = request.message.clone();
    
    if let (Some(project_name), Some(file_name), Some(file_path)) = 
        (&request.project_name, &request.selected_file_name, &request.selected_file_path) {
        prompt = format!(
            "Context: You are an AI assistant helping with the '{}' project. The user is currently viewing the file '{}' located at '{}'.\n\nUser question: {}\n\nPlease provide a helpful, detailed response about this specific file or the project in general. Focus on explaining the code, architecture, best practices, and any relevant insights. Be direct and factual without any thinking process.",
            project_name, file_name, file_path, request.message
        );
    } else if let Some(project_name) = &request.project_name {
        prompt = format!(
            "Context: You are an AI assistant helping with the '{}' project.\n\nUser question: {}\n\nPlease provide a helpful, detailed response about this project. If the user is asking about project structure, explain the typical structure of such projects and suggest they select a specific file for more detailed analysis. If they're asking about code, suggest they click on a file to get specific information. Be direct and factual without any thinking process.",
            project_name, request.message
        );
    } else {
        prompt = format!(
            "User question: {}\n\nYou are an AI assistant for code analysis. Please provide a helpful response. If the user is asking about project structure, explain that you need more context and suggest they upload a project or select a specific file for detailed analysis. Be direct and factual without any thinking process.",
            request.message
        );
    }

    let response = ai_service.analyze_text(&prompt)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    info!("Chat response generated successfully");
    Ok(HttpResponse::Ok().json(ChatResponse { response }))
} 