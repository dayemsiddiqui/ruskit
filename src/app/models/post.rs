use validator::ValidationError;
use crate::framework::database::{
    model::{Model, BelongsTo, Rules, Validate, ValidationRules},
    query_builder::QueryBuilder,
    DatabaseError,
    migration::Migration,
};
use crate::app::entities::{User, Post};
use async_trait::async_trait;

impl Post {
    /// Get the user who created this post
    pub fn user(&self) -> BelongsTo<User> {
        BelongsTo::new::<Self>()
    }

    /// Get recent records
    pub async fn recent(limit: i64) -> Result<Vec<Self>, DatabaseError> {
        QueryBuilder::table(Self::table_name())
            .order_by("created_at", "DESC")
            .limit(limit)
            .get::<Self>()
            .await
    }
}

impl ValidationRules for Post {
    fn validate_rules(&self) -> Result<(), ValidationError> {
        self.title.validate(Rules::new().required().max(255))?;
        self.content.validate(Rules::new().required())?;
        Ok(())
    }
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
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                )",
                "DROP TABLE posts"
            ),
            Migration::new(
                "1740006219_add_user_id_column",
                "ALTER TABLE posts ADD COLUMN user_id INTEGER NOT NULL",
                "ALTER TABLE posts DROP COLUMN user_id"
            ),
            Migration::new(
                "1740006309_add_other_post_attributes",
                "ALTER TABLE posts ADD COLUMN title TEXT NOT NULL; ALTER TABLE posts ADD COLUMN content TEXT NOT NULL",
                "ALTER TABLE posts DROP COLUMN content; ALTER TABLE posts DROP COLUMN title"
            ),
        ]
    }
}