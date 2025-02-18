use async_trait::async_trait;
use serde_json::Value;
use crate::framework::database::model::Model;
use crate::framework::database::DatabaseError;

#[async_trait]
pub trait Factory: Model {
    fn definition() -> Value;
    
    async fn factory() -> Result<Self, DatabaseError> {
        Self::create(Self::definition()).await
    }
    
    async fn create_many(count: i32) -> Result<Vec<Self>, DatabaseError> {
        let mut records = Vec::new();
        for _ in 0..count {
            records.push(Self::factory().await?);
        }
        Ok(records)
    }
} 