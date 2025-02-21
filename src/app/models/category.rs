use crate::framework::prelude::*;
use crate::app::entities::Category;

impl Category {
    /// Get recent records
    pub async fn recent(limit: i64) -> Result<Vec<Self>, DatabaseError> {
        QueryBuilder::table(Self::table_name())
            .order_by("created_at", "DESC")
            .limit(limit)
            .get::<Self>()
            .await
    }
}

impl ValidationRules for Category {
    fn validate_rules(&self) -> Result<(), ValidationError> {
        // TODO: Add your validation rules here
        Ok(())
    }
}

#[async_trait]
impl Model for Category {
    fn table_name() -> &'static str {
        "categories"
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
            Migration::create("1740104269_create_categories_table", |schema| {
                schema.create_table("categories", |table| {
                    table.id();
                    // TODO: Add your columns here
                    table.timestamps();
                });
            })
            .down(|schema| {
                schema.drop_table("categories");
            })
        ]
    }
}