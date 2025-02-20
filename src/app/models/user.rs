use validator::ValidationError;
use crate::framework::database::{
    model::{Model, HasMany, Rules, Validate, ValidationRules},
    query_builder::QueryBuilder,
    DatabaseError,
    migration::Migration,
};
use crate::app::entities::Post;
use crate::app::entities::User;
use async_trait::async_trait;

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