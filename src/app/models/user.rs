use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;
use crate::framework::database::{
    model::{Model, HasMany},
    query_builder::QueryBuilder,
    DatabaseError,
    migration::Migration,
};
use crate::app::models::post::Post;
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct User {
    #[sqlx(default)]
    pub id: i64,
    #[validate(length(min = 3, max = 255))]
    #[serde(deserialize_with = "deserialize_unquoted_string")]
    pub name: String,
    #[validate(email)]
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