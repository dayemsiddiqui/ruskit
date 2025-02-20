use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rustavel_derive::GenerateValidationFields;
use crate::framework::database::model::{Field, ModelValidation};
use validator::ValidationError;

#[derive(Debug, Serialize, Deserialize, FromRow, GenerateValidationFields)]
pub struct Post {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
} 