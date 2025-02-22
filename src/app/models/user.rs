use crate::framework::prelude::*;
use crate::app::entities::User;

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
}