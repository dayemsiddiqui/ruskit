use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::framework::database::{
    model::Model,
    query_builder::QueryBuilder,
    DatabaseError,
    migration::Migration,
};
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
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
                "create_users_table",
                "CREATE TABLE users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    email TEXT NOT NULL UNIQUE,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                )",
                "DROP TABLE users"
            )
        ]
    }
}

impl User {
    pub async fn by_email(email: &str) -> Result<Option<Self>, DatabaseError> {
        QueryBuilder::table(Self::table_name())
            .where_clause("email", "=", email)
            .first::<Self>()
            .await
    }

    pub async fn all() -> Result<Vec<Self>, DatabaseError> {
        QueryBuilder::table(Self::table_name())
            .get::<Self>()
            .await
    }

    pub async fn find(id: i64) -> Result<Option<Self>, DatabaseError> {
        QueryBuilder::table(Self::table_name())
            .where_clause("id", "=", id)
            .first::<Self>()
            .await
    }

    pub async fn create(attributes: serde_json::Value) -> Result<Self, DatabaseError> {
        Model::create(attributes).await
    }
} 