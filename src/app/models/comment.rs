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
pub struct Comment {
    pub id: i64,
    // TODO: Add your fields here
    pub created_at: i64,
    pub updated_at: i64,
}

#[async_trait]
impl Model for Comment {
    fn table_name() -> &'static str {
        "comments"
    }

    fn migrations() -> Vec<Migration> {
        vec![
            Migration::new(
                "1739885732_create_comments_table",
                "CREATE TABLE comments (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    -- TODO: Add your columns here
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                )",
                "DROP TABLE comments"
            ),
        ]
    }
}

impl Comment {
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