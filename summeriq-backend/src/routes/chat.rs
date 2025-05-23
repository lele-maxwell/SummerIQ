use axum::{
    extract::State,
    routing::post,
    Router,
    Json,
};
use crate::{
    error::AppError,
    models::message::{Message, CreateMessage},
    services::ai::AIService,
};
use sqlx::PgPool;
use uuid::Uuid;

pub fn chat_router() -> Router<(PgPool, AIService)> {
    Router::new()
        .route("/ask", post(ask))
}

async fn ask(
    State((pool, ai)): State<(PgPool, AIService)>,
    Json(payload): Json<CreateMessage>,
) -> Result<Json<Message>, AppError> {
    // Get file content from database
    let files = sqlx::query!(
        r#"
        SELECT path, summary FROM files
        WHERE upload_id = $1
        "#,
        payload.upload_id
    )
    .fetch_all(&pool)
    .await?;

    // Combine file contents for context
    let context = files
        .iter()
        .map(|f| format!("File: {}\nContent: {}", f.path, f.summary))
        .collect::<Vec<_>>()
        .join("\n\n");

    // Get AI response
    let answer = ai.answer_question(&context, &payload.question).await?;

    // Save message
    let message = sqlx::query_as!(
        Message,
        r#"
        INSERT INTO messages (user_id, upload_id, question, answer)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        Uuid::new_v4(), // TODO: Get from auth
        payload.upload_id,
        payload.question,
        answer
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(message))
}
