use std::sync::Arc;
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use once_cell::sync::OnceCell;
use thiserror::Error;
use std::fs;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::sync::RwLock;
use once_cell::sync::Lazy;

pub mod model;
pub mod query_builder;
pub mod migration;
pub mod config;
pub mod seeder;
pub mod factory;

static POOL: Lazy<RwLock<Option<Arc<SqlitePool>>>> = Lazy::new(|| RwLock::new(None));

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    ConnectionError(#[from] sqlx::Error),
    #[error("Database not initialized")]
    NotInitialized,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Initialize the database connection pool
pub async fn initialize(config: Option<config::DatabaseConfig>) -> Result<(), DatabaseError> {
    let config = config.unwrap_or_else(config::DatabaseConfig::from_env);
    config::set_config(config.clone());
    
    let db_path = &config.connections.sqlite.database;
    println!("Initializing database at path: {}", db_path);
    
    // Create database directory if it doesn't exist
    if let Some(parent) = Path::new(db_path).parent() {
        println!("Creating database directory: {}", parent.display());
        fs::create_dir_all(parent).map_err(|e| 
            DatabaseError::ConnectionError(sqlx::Error::Configuration(
                format!("Failed to create database directory: {}", e).into()
            ))
        )?;
        
        // Set directory permissions to 755 (rwxr-xr-x)
        let metadata = fs::metadata(parent).map_err(|e|
            DatabaseError::ConnectionError(sqlx::Error::Configuration(
                format!("Failed to get directory metadata: {}", e).into()
            ))
        )?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(parent, perms).map_err(|e|
            DatabaseError::ConnectionError(sqlx::Error::Configuration(
                format!("Failed to set directory permissions: {}", e).into()
            ))
        )?;
    }

    // Create an empty database file if it doesn't exist
    if !Path::new(db_path).exists() {
        println!("Creating database file");
        fs::File::create(db_path).map_err(|e|
            DatabaseError::ConnectionError(sqlx::Error::Configuration(
                format!("Failed to create database file: {}", e).into()
            ))
        )?;
        
        // Set file permissions to 644 (rw-r--r--)
        let metadata = fs::metadata(db_path).map_err(|e|
            DatabaseError::ConnectionError(sqlx::Error::Configuration(
                format!("Failed to get file metadata: {}", e).into()
            ))
        )?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o644);
        fs::set_permissions(db_path, perms).map_err(|e|
            DatabaseError::ConnectionError(sqlx::Error::Configuration(
                format!("Failed to set file permissions: {}", e).into()
            ))
        )?;
    }

    let connection_url = config.connection_url();
    println!("Connecting to database with URL: {}", connection_url);
    
    let pool = SqlitePool::connect(&connection_url).await?;
    println!("Successfully connected to database");
    
    // Enable foreign key constraints if configured
    if config.connections.sqlite.foreign_key_constraints {
        println!("Enabling foreign key constraints");
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await?;
    }
    
    POOL.write().unwrap().replace(Arc::new(pool));
    println!("Database pool initialized successfully");
    Ok(())
}

/// Get a reference to the database pool
pub fn get_pool() -> Result<Arc<SqlitePool>, DatabaseError> {
    POOL.read()
        .unwrap()
        .as_ref()
        .cloned()
        .ok_or(DatabaseError::NotInitialized)
}

pub fn set_pool(pool: SqlitePool) -> Result<(), DatabaseError> {
    let mut pool_guard = POOL.write().unwrap();
    *pool_guard = Some(Arc::new(pool));
    Ok(())
} 