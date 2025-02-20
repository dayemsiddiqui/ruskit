use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::ValidationError;
use crate::framework::database::{
    model::{Model, HasMany, ModelValidation, Field, Rules, Validate},
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
    #[serde(deserialize_with = "deserialize_unquoted_string")]
    pub name: String,
    #[serde(deserialize_with = "deserialize_unquoted_string")]
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
}

fn deserialize_unquoted_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.trim_matches('"').to_string())
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

impl ModelValidation for User {
    type Fields = UserFields;

    fn fields() -> Self::Fields {
        UserFields::new()
    }

    fn validate(&self) -> Result<(), ValidationError> {
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