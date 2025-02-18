use clap::Parser;
use ruskit::app;
use ruskit::framework::database;
use ruskit::framework::database::migration::MigrationManager;
use ruskit::framework::database::seeder::DatabaseSeeder;
use ruskit::app::seeders::user_seeder;

#[derive(Parser)]
pub struct DbSeed;

impl DbSeed {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting seeder...");
        println!("Initializing application...");
        app::initialize();

        println!("Initializing database...");
        let db_config = database::config::DatabaseConfig::from_env();
        database::initialize(Some(db_config)).await?;

        println!("Initializing seeders...");
        user_seeder::initialize();

        println!("Seeding database...");
        let seeders = DatabaseSeeder::run_all().await;
        match seeders {
            Ok(_) => {
                println!("Database seeded successfully!");
                Ok(())
            },
            Err(e) => {
                println!("Error seeding database: {}", e);
                Err(Box::new(e))
            }
        }
    }
} 