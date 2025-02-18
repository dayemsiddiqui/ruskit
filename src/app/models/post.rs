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
pub struct Post {
    pub id: i64,
    // TODO: Add your fields here
    pub created_at: i64,
    pub updated_at: i64,
}

#[async_trait]
impl Model for Post {
    fn id(&self) -> i64 {
        self.id
    }

    fn table_name() -> &'static str {
        "posts"
    }

    fn migrations() -> Vec<Migration> {
        vec![
            Migration::new(
                "1739907242_create_posts_table",
                "CREATE TABLE posts (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    -- TODO: Add your columns here
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                )",
                "DROP TABLE posts"
            ),
        ]
    }
}

impl Post {
    // TODO: Add your custom query methods here
    
    /// Get recent records
    pub async fn recent(limit: i64) -> Result<Vec<Self>, DatabaseError> {
        QueryBuilder::table(Self::table_name())
            .order_by("created_at", "DESC")
            .limit(limit)
            .get::<Self>()
            .await
    }
}