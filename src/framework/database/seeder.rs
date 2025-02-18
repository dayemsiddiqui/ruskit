use async_trait::async_trait;
use crate::framework::database::DatabaseError;
use std::sync::Mutex;
use once_cell::sync::Lazy;

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
        
        let mut errors = Vec::new();
        
        for (i, seeder) in seeders.iter().enumerate() {
            println!("Running seeder {} of {}...", i + 1, count);
            match seeder.run().await {
                Ok(_) => println!("Seeder {} completed successfully", i + 1),
                Err(e) => {
                    println!("Seeder {} failed: {}", i + 1, e);
                    errors.push(format!("Seeder {} failed: {}", i + 1, e));
                }
            }
        }

        if errors.is_empty() {
            println!("All seeders completed successfully");
            Ok(())
        } else {
            println!("Some seeders failed:");
            for error in &errors {
                println!("  - {}", error);
            }
            Err(DatabaseError::Other(format!("{} out of {} seeders failed", errors.len(), count)))
        }
    }
} 