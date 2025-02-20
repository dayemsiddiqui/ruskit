use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::ValidationError;
use crate::framework::database::{
    model::{Model, HasMany, ModelValidation, Field, Rules, Validate, ValidationRules},
    query_builder::QueryBuilder,
    DatabaseError,
    migration::Migration,
};
use crate::app::models::post::Post;
use async_trait::async_trait;
use rustavel_derive::GenerateValidationFields;

#[derive(Debug, Serialize, Deserialize, FromRow, GenerateValidationFields)]
pub struct User {
    #[sqlx(default)]
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl User {
    /// Get recent records
    pub async fn recent(limit: i64) -> Result<Vec<Self>, DatabaseError> {
        QueryBuilder::table(Self::table_name())
            .order_by("created_at", "DESC")
            .limit(limit)
            .get::<Self>()
            .await
    }

    /// Get all posts by this user
    pub fn posts(&self) -> HasMany<Post> {
        HasMany::new::<Self>()
    }
}

impl ValidationRules for User {
    fn validate_rules(&self) -> Result<(), ValidationError> {
        self.name.validate(Rules::new().required().min(3).max(255))?;
        self.email.validate(Rules::new().required().email())?;
        Ok(())
    }
}

#[async_trait]
impl Model for User {
    fn table_name() -> &'static str {
        "users"
    }

    fn id(&self) -> i64 {
        self.id
    }

    fn migrations() -> Vec<Migration> {
        vec![
            Migration::new(
                "1739887638_create_users_table",
                "CREATE TABLE users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    email TEXT NOT NULL,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                )",
                "DROP TABLE users"
            ),
        ]
    }
}