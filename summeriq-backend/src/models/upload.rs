use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Upload {
    pub id: Uuid,
    pub user_id: Uuid,
    pub file_name: String,
    pub minio_key: String,
    pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUpload {
    pub file_name: String,
}
