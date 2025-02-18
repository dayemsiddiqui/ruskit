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

static SEEDERS: Lazy<Mutex<Vec<Box<dyn Seeder + Send + Sync>>>> = Lazy::new(|| {
    println!("Initializing seeders vector");
    Mutex::new(Vec::new())
});

impl DatabaseSeeder {
    pub fn register(seeder: Box<dyn Seeder + Send + Sync>) {
        println!("Registering seeder in DatabaseSeeder...");
        let mut seeders = SEEDERS.lock().unwrap();
        seeders.push(seeder);
        let count = seeders.len();
        println!("Total registered seeders: {}", count);
    }

    fn discover_seeders() -> Result<(), DatabaseError> {
        // Seeders are auto-registered via static initializers
        // We don't need to manually load them
        Ok(())
    }

    pub async fn run_all() -> Result<(), DatabaseError> {
        let seeders = SEEDERS.lock().unwrap();
        let count = seeders.len();
        println!("Running {} seeders...", count);
        
        for seeder in seeders.iter() {
            println!("Running seeder...");
            match seeder.run().await {
                Ok(_) => println!("Seeder completed successfully"),
                Err(e) => {
                    println!("Seeder failed: {}", e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }
} 