use async_trait::async_trait;
use crate::framework::database::DatabaseError;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::fs;
use std::path::Path;

#[async_trait]
pub trait Seeder: Send + Sync {
    async fn run(&self) -> Result<(), DatabaseError>;
}

pub struct DatabaseSeeder;

static SEEDERS: Lazy<Mutex<Vec<Box<dyn Seeder + Send + Sync>>>> = Lazy::new(|| Mutex::new(Vec::new()));

impl DatabaseSeeder {
    pub fn register(seeder: Box<dyn Seeder + Send + Sync>) {
        SEEDERS.lock().unwrap().push(seeder);
    }

    fn discover_seeders() -> Result<(), DatabaseError> {
        // Seeders are auto-registered via static initializers
        // We don't need to manually load them
        Ok(())
    }

    pub async fn run_all() -> Result<(), DatabaseError> {
        Self::discover_seeders()?;
        let seeders = SEEDERS.lock().unwrap();
        for seeder in seeders.iter() {
            seeder.run().await?;
        }
        Ok(())
    }
} 