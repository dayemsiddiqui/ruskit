use crate::framework::database::{model::Model, DatabaseError};

pub trait Factory: Model {
    fn definition() -> Self;
    
    async fn factory() -> Result<Self, DatabaseError> {
        let instance = Self::definition();
        println!("Creating factory instance...");
        Self::create(instance).await
    }
    
    async fn create_many(count: i32) -> Result<Vec<Self>, DatabaseError> {
        println!("Creating {} instances...", count);
        let mut records = Vec::new();
        let mut errors = Vec::new();
        
        for i in 0..count {
            println!("Creating instance {}...", i + 1);
            match Self::factory().await {
                Ok(record) => {
                    println!("Successfully created instance {} with ID {}", i + 1, record.id());
                    records.push(record);
                },
                Err(e) => {
                    println!("Failed to create instance {}: {}", i + 1, e);
                    errors.push(format!(
                        "Failed to create instance {} of {}: {}",
                        i + 1,
                        std::any::type_name::<Self>(),
                        e
                    ));
                }
            }
        }
        
        if !errors.is_empty() {
            println!("Some instances failed to create:");
            for error in &errors {
                println!("  - {}", error);
            }
        }
        
        println!("Successfully created {} out of {} instances", records.len(), count);
        if !records.is_empty() {
            Ok(records)
        } else {
            Err(DatabaseError::Other(format!("Failed to create any instances. Errors: {:?}", errors)))
        }
    }
} 