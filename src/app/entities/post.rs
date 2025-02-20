// Framework imports from prelude
use crate::app::prelude::*;

#[derive(Debug, Serialize, Deserialize, FromRow, GenerateValidationFields)]
pub struct Post {
    #[sqlx(default)]
    pub id: i64,
    pub title: String,
    pub content: String,
    pub user_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
} 