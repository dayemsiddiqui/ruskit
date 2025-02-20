use crate::framework::prelude::*;
use crate::app::entities::User;
use fake::{Fake, Faker};

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

impl ValidationRules for User {
    fn validate_rules(&self) -> Result<(), ValidationError> {
        // TODO: Add your validation rules here
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

    fn factory_definition() -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: 0,
            name: Faker.fake(),
            email: Faker.fake(),
            created_at: now.to_string(),
            updated_at: now.to_string(),
        }
    }

    fn migrations() -> Vec<Migration> {
        vec![
            Migration::create("1739887638_create_users_table", |schema| {
                schema.create_table("users", |table| {
                    table.id();
                    table.text("name").not_null();
                    table.text("email").not_null();
                    table.timestamp_iso_strings();
                });
            })
            .down(|schema| {
                schema.drop_table("users");
            })
        ]
    }
}