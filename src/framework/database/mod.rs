use std::sync::Arc;
use sqlx::{sqlite::{SqlitePool, SqlitePoolOptions}};
use thiserror::Error;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub mod model;
pub mod query_builder;
pub mod migration;
pub mod config;
pub mod seeder;
pub mod factory;

use config::DatabaseConfig;
use seeder::DatabaseSeeder;

pub static POOL: Lazy<Mutex<Option<Arc<SqlitePool>>>> = Lazy::new(|| Mutex::new(None));

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    ConnectionError(#[from] sqlx::Error),
    #[error("Database not initialized")]
    NotInitialized,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

/// Initialize the database connection pool
pub async fn initialize(config: Option<DatabaseConfig>) -> Result<(), DatabaseError> {
    let config = config.unwrap_or_else(|| DatabaseConfig::default());
    let database_path = config.database_path();
    let db_path = Path::new(&database_path);

    // Create database directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        println!("Creating database directory: {}", parent.display());
        fs::create_dir_all(parent)?;
    }

    println!("Connecting to database with URL: {}", config.connection_url());
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&config.connection_url())
        .await?;

    println!("Successfully connected to database");

    // Enable foreign key constraints
    println!("Enabling foreign key constraints");
    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await?;

    println!("Database pool initialized successfully");

    *POOL.lock().unwrap() = Some(Arc::new(pool));

    Ok(())
}

/// Get a reference to the database pool
pub fn get_pool() -> Result<Arc<SqlitePool>, DatabaseError> {
    println!("Getting database pool...");
    let pool = POOL.lock()
        .unwrap()
        .as_ref()
        .cloned()
        .ok_or(DatabaseError::ConnectionError(
            sqlx::Error::Configuration("Database not initialized".into())
        ))?;
    println!("Got database pool successfully");
    Ok(pool)
}

pub async fn seed() -> Result<(), DatabaseError> {
    println!("Starting database seeding...");
    println!("Running seeders...");
    match DatabaseSeeder::run_all().await {
        Ok(_) => {
            println!("All seeders completed successfully");
            Ok(())
        },
        Err(e) => {
            println!("Error running seeders: {}", e);
            Err(e)
        }
    }
} 