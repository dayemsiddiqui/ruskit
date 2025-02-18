use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{FromRow, sqlite::SqliteRow};
use crate::framework::database::{get_pool, DatabaseError};
use crate::framework::database::migration::Migration;
use std::sync::Mutex;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::fs;
use std::path::Path;
use serde_json::Value;
use sqlx::sqlite::SqlitePool;
use crate::framework::database::factory::Factory;

type MigrationFn = fn() -> Vec<Migration>;

static MODEL_REGISTRY: Lazy<Mutex<HashMap<String, MigrationFn>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn register_model_with_migrations(model_name: String, migrations_fn: MigrationFn) {
    MODEL_REGISTRY.lock().unwrap().insert(model_name, migrations_fn);
}

pub fn get_all_model_migrations() -> Vec<Migration> {
    let registry = MODEL_REGISTRY.lock().unwrap();
    let mut migrations = Vec::new();
    
    for migrations_fn in registry.values() {
        migrations.extend(migrations_fn());
    }
    
    // Sort migrations by timestamp prefix to ensure chronological order
    migrations.sort_by(|a, b| {
        let a_timestamp = a.name.split('_').next().unwrap_or("0")
            .parse::<u64>().unwrap_or(0);
        let b_timestamp = b.name.split('_').next().unwrap_or("0")
            .parse::<u64>().unwrap_or(0);
        a_timestamp.cmp(&b_timestamp)
    });
    
    migrations
}

/// Automatically discover and register all models in the models directory
pub fn discover_and_register_models() -> std::io::Result<()> {
    let models_dir = Path::new("src/app/models");
    if !models_dir.exists() {
        return Ok(());
    }

    // Read all entries in the models directory
    for entry in fs::read_dir(models_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        // Skip mod.rs and non-rust files
        if path.is_file() && 
           path.extension().map_or(false, |ext| ext == "rs") && 
           path.file_name().map_or(false, |name| name != "mod.rs") {
            // Get the model name from the file name
            if let Some(model_name) = path.file_stem().and_then(|s| s.to_str()) {
                // Convert to PascalCase for the struct name
                let model_name = model_name.chars().next().unwrap_or('_').to_uppercase().to_string() + 
                               &model_name[1..];
                let full_type_name = format!("ruskit::app::models::{}", model_name);
                
                // The model will register itself when it's used
                println!("Discovered model: {}", full_type_name);
            }
        }
    }
    
    Ok(())
}

#[async_trait]
pub trait Model: for<'r> FromRow<'r, SqliteRow> + Serialize + DeserializeOwned + Send + Sync + Sized + Unpin {
    /// Get the table name for the model
    fn table_name() -> &'static str;

    /// Get the primary key name (defaults to "id")
    fn primary_key() -> &'static str {
        "id"
    }

    /// Get the migrations for this model
    fn migrations() -> Vec<Migration>;

    /// Register this model in the registry
    fn register() {
        register_model_with_migrations(
            std::any::type_name::<Self>().to_string(),
            Self::migrations
        );
    }

    /// Find a model by its primary key
    async fn find(id: i64) -> Result<Option<Self>, DatabaseError> {
        let pool = get_pool()?;
        let query = format!(
            "SELECT * FROM {} WHERE {} = ?",
            Self::table_name(),
            Self::primary_key()
        );
        
        let result = sqlx::query_as(&query)
            .bind(id)
            .fetch_optional(pool.as_ref())
            .await?;
            
        Ok(result)
    }

    /// Get all records
    async fn all() -> Result<Vec<Self>, DatabaseError> {
        let pool = get_pool()?;
        let query = format!("SELECT * FROM {}", Self::table_name());
        
        let results = sqlx::query_as(&query)
            .fetch_all(pool.as_ref())
            .await?;
            
        Ok(results)
    }

    /// Create a new record
    async fn create(data: Value) -> Result<Self, DatabaseError> {
        println!("Creating new record...");
        let pool = get_pool()?;
        let columns: Vec<String> = data.as_object().unwrap().keys().cloned().collect();
        let values: Vec<Value> = data.as_object().unwrap().values().cloned().collect();
        let placeholders: Vec<String> = (1..=values.len()).map(|_| format!("?")).collect();

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            Self::table_name(),
            columns.join(", "),
            placeholders.join(", ")
        );

        println!("Executing query: {}", query);
        println!("With values: {:?}", values);

        let mut query_builder = sqlx::query(&query);
        for value in values {
            query_builder = query_builder.bind(value.to_string());
        }

        query_builder.execute(&*pool).await?;

        let last_id = sqlx::query_scalar::<_, i64>("SELECT last_insert_rowid()")
            .fetch_one(&*pool)
            .await?;

        println!("Record created with ID: {}", last_id);

        let result = sqlx::query_as::<_, Self>(&format!(
            "SELECT * FROM {} WHERE id = ?",
            Self::table_name()
        ))
        .bind(last_id)
        .fetch_one(&*pool)
        .await?;

        println!("Record retrieved successfully");
        Ok(result)
    }
} 