use crate::framework::prelude::*;
use crate::app::entities::Post;

impl Post {
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
        // TODO: Add your validation rules here
        Ok(())
    }
}

#[async_trait]
impl Model for Post {
    fn table_name() -> &'static str {
        "posts"
    }

    fn id(&self) -> i64 {
        self.id
    }

    fn factory_definition() -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: 0,
            // TODO: Add your fake data here using Faker
            // Example: name: Faker.fake(),
            created_at: now,
            updated_at: now,
        }
    }

    fn migrations() -> Vec<Migration> {
        vec![
            Migration::create("1740104021_create_posts_table", |schema| {
                schema.create_table("posts", |table| {
                    table.id();
                    // TODO: Add your columns here
                    table.timestamps();
                });
            })
            .down(|schema| {
                schema.drop_table("posts");
            })
        ,
            Migration::create("1740104086_create_posts", |schema| {
                // TODO: Add your migration schema changes here
                // Example:
                // schema.create_table("table_name", |table| {
                //     table.id();
                //     table.text("name").not_null();
                //     table.timestamp_iso_strings();
                // });
            })
            .down(|schema| {
                // TODO: Add your rollback schema changes here
                // Example:
                // schema.drop_table("table_name");
            })]
    }
}