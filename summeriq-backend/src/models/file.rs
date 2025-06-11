use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub id: Uuid,
    pub user_id: Uuid,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub size: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFile {
    pub user_id: Uuid,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub size: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FileAnalysis {
    pub summary: String,
    pub functions: Vec<Function>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: Option<serde_json::Value>,
    pub return_type: Option<String>,
}
