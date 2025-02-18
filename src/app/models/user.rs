use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;
use crate::framework::database::{
    model::Model,
    query_builder::QueryBuilder,
    DatabaseError,
    migration::Migration,
};
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct User {
    pub id: i64,
    #[validate(length(min = 3, max = 255))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[async_trait]
impl Model for User {
    fn table_name() -> &'static str {
        "users"
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
}