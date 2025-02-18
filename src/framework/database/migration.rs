use sqlx::{SqlitePool, Row};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::framework::database::{
    get_pool,
    DatabaseError,
    config,
};
use crate::framework::database::model::{Model, discover_and_register_models, get_all_model_migrations};
use crate::app::models::*;
use sqlx::query;
use sqlx::{Pool, Sqlite};
use std::fs;
use std::path::Path;

pub struct Migration {
    pub name: String,
    pub up: String,
    pub down: String,
}

impl Migration {
    pub fn new(name: &str, up: &str, down: &str) -> Self {
        Self {
            name: name.to_string(),
            up: up.to_string(),
            down: down.to_string(),
        }
    }
}

pub struct MigrationManager {
    pool: Pool<Sqlite>,
}

impl MigrationManager {
    pub async fn new() -> Result<Self, DatabaseError> {
        // Initialize database if not already initialized
        let db_config = config::DatabaseConfig::from_env();
        let pool = SqlitePool::connect(&db_config.connection_url()).await?;
        
        // Discover and register all models
        discover_and_register_models().map_err(|e| 
            DatabaseError::ConnectionError(sqlx::Error::Configuration(
                format!("Failed to discover models: {}", e).into()
            ))
        )?;
        
        
        Ok(Self { pool })
    }

    pub async fn setup_migration_table(&self) -> Result<(), DatabaseError> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS migrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                batch INTEGER NOT NULL,
                migration_time INTEGER NOT NULL
            )"
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn get_all_migrations(&self) -> Result<Vec<String>, DatabaseError> {
        let migrations = sqlx::query("SELECT name FROM migrations ORDER BY id")
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|row| row.try_get("name").unwrap())
            .collect();
            
        Ok(migrations)
    }

    pub async fn get_last_batch(&self) -> Result<i32, DatabaseError> {
        let result: Option<i32> = sqlx::query_scalar("SELECT MAX(batch) FROM migrations")
            .fetch_optional(&self.pool)
            .await?;
            
        Ok(result.unwrap_or(0))
    }

    pub async fn run(&self, migrations: Vec<Migration>) -> Result<(), DatabaseError> {
        // Create migrations table if it doesn't exist
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS migrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                batch INTEGER NOT NULL,
                created_at INTEGER NOT NULL
            )"
        )
        .execute(&self.pool)
        .await?;

        // Get the last batch number
        let last_batch: i64 = sqlx::query_scalar("SELECT COALESCE(MAX(batch), 0) FROM migrations")
            .fetch_one(&self.pool)
            .await?;

        let current_batch = last_batch + 1;

        // Get list of already run migrations
        let executed_migrations: Vec<String> = sqlx::query_scalar("SELECT name FROM migrations")
            .fetch_all(&self.pool)
            .await?;

        // Run pending migrations
        for migration in migrations {
            if !executed_migrations.contains(&migration.name) {
                println!("Running migration: {}", migration.name);

                // Begin transaction
                let mut tx = self.pool.begin().await?;

                // Run the migration
                sqlx::query(&migration.up)
                    .execute(&mut *tx)
                    .await?;

                // Record the migration
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;

                sqlx::query(
                    "INSERT INTO migrations (name, batch, created_at) VALUES (?, ?, ?)"
                )
                .bind(&migration.name)
                .bind(current_batch)
                .bind(now)
                .execute(&mut *tx)
                .await?;

                // Commit transaction
                tx.commit().await?;

                println!("Migration completed: {}", migration.name);
            }
        }

        Ok(())
    }

    pub async fn rollback(&self) -> Result<(), DatabaseError> {
        // Get the last batch number
        let last_batch = self.get_last_batch().await?;
        
        // Get migrations from the last batch
        let migrations = sqlx::query(
            "SELECT name FROM migrations WHERE batch = ? ORDER BY id DESC"
        )
        .bind(last_batch)
        .fetch_all(&self.pool)
        .await?;
        
        // Get all model migrations to find the down SQL
        let model_migrations = self.get_all_model_migrations();
        
        for row in migrations {
            let name: String = row.try_get("name")?;
            println!("Rolling back migration: {}", name);
            
            // Find the corresponding migration
            if let Some(migration) = model_migrations.iter().find(|m| m.name == name) {
                // Run the down migration
                sqlx::query(&migration.down)
                    .execute(&self.pool)
                    .await?;
                
                // Delete the migration record
                sqlx::query("DELETE FROM migrations WHERE name = ?")
                    .bind(&name)
                    .execute(&self.pool)
                    .await?;

                println!("Rollback completed: {}", name);
            }
        }
        
        Ok(())
    }

    pub async fn refresh(&self) -> Result<(), DatabaseError> {
        // Drop all tables
        println!("Dropping all tables...");
        
        // Get all tables
        let tables = sqlx::query(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'"
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Drop each table
        for row in tables {
            let table: String = row.try_get("name")?;
            sqlx::query(&format!("DROP TABLE IF EXISTS {}", table))
                .execute(&self.pool)
                .await?;
        }
        
        println!("All tables dropped.");
        
        // Run all migrations
        println!("Running all migrations...");
        self.run(self.get_all_model_migrations()).await?;
        println!("All migrations completed.");
        
        Ok(())
    }

    pub fn get_all_model_migrations(&self) -> Vec<Migration> {
        crate::app::models::get_all_model_migrations()
    }

    pub async fn drop_all_tables(&self) -> Result<(), DatabaseError> {
        // Begin transaction
        let mut tx = self.pool.begin().await?;

        // Disable foreign key checks temporarily
        sqlx::query("PRAGMA foreign_keys = OFF")
            .execute(&mut *tx)
            .await?;

        // Get all table names
        let tables: Vec<String> = sqlx::query_scalar(
            "SELECT name FROM sqlite_master WHERE type='table' AND name != 'sqlite_sequence'"
        )
        .fetch_all(&mut *tx)
        .await?;

        // Drop each table
        for table in tables {
            println!("Dropping table: {}", table);
            sqlx::query(&format!("DROP TABLE IF EXISTS {}", table))
                .execute(&mut *tx)
                .await?;
        }

        // Re-enable foreign key checks
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&mut *tx)
            .await?;

        // Commit transaction
        tx.commit().await?;

        Ok(())
    }
}

// Helper macro for creating migrations
#[macro_export]
macro_rules! create_migration {
    ($name:expr, $up:expr, $down:expr) => {
        Migration::new($name, $up, $down)
    };
} 