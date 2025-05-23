use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct File {
    pub id: Uuid,
    pub upload_id: Uuid,
    pub path: String,
    pub summary: Option<String>,
    pub functions: Option<serde_json::Value>,
    pub dependencies: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
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
