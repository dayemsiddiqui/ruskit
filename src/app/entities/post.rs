use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rustavel_derive::GenerateValidationFields;

#[derive(Debug, Serialize, Deserialize, FromRow, GenerateValidationFields)]
pub struct Post {
    #[sqlx(default)]
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
} 