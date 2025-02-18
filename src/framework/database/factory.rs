use async_trait::async_trait;
use serde_json::Value;
use crate::framework::database::model::Model;
use crate::framework::database::DatabaseError;

pub trait Factory: Model {
    fn definition() -> Value;
    
    async fn factory() -> Result<Self, DatabaseError> {
        println!("Creating factory instance...");
        let data = Self::definition();
        println!("Factory data: {:?}", data);
        Self::create(data).await
    }
    
    async fn create_many(count: i32) -> Result<Vec<Self>, DatabaseError> {
        println!("Creating {} instances...", count);
        let mut records = Vec::new();
        for i in 0..count {
            println!("Creating instance {}...", i + 1);
            records.push(Self::factory().await?);
        }
        println!("Successfully created {} instances", count);
        Ok(records)
    }
} 