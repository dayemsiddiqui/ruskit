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
use std::marker::PhantomData;

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

pub struct BelongsTo<Parent: Model, Child: Model> {
    parent_type: PhantomData<Parent>,
    child_type: PhantomData<Child>,
    foreign_key: &'static str,
}

pub struct HasMany<Parent: Model, Child: Model> {
    parent_type: PhantomData<Parent>,
    child_type: PhantomData<Child>,
    foreign_key: &'static str,
}

pub struct HasOne<Parent: Model, Child: Model> {
    parent_type: PhantomData<Parent>,
    child_type: PhantomData<Child>,
    foreign_key: &'static str,
}

impl<Parent: Model, Child: Model> BelongsTo<Parent, Child> {
    pub fn new(foreign_key: &'static str) -> Self {
        Self {
            parent_type: PhantomData,
            child_type: PhantomData,
            foreign_key,
        }
    }

    pub async fn get(&self, model: &Child) -> Result<Option<Parent>, DatabaseError> {
        let foreign_key_value = model.get_field_value(self.foreign_key)?;
        Parent::find(foreign_key_value).await
    }
}

impl<Parent: Model, Child: Model> HasMany<Parent, Child> {
    pub fn new(foreign_key: &'static str) -> Self {
        Self {
            parent_type: PhantomData,
            child_type: PhantomData,
            foreign_key,
        }
    }

    pub async fn get(&self, model: &Parent) -> Result<Vec<Child>, DatabaseError> {
        let pool = get_pool()?;
        let query = format!(
            "SELECT * FROM {} WHERE {} = ?",
            Child::table_name(),
            self.foreign_key
        );
        
        let results = sqlx::query_as::<sqlx::Sqlite, Child>(&query)
            .bind(model.id())
            .fetch_all(pool.as_ref())
            .await?;
            
        Ok(results)
    }
}

impl<Parent: Model, Child: Model> HasOne<Parent, Child> {
    pub fn new(foreign_key: &'static str) -> Self {
        Self {
            parent_type: PhantomData,
            child_type: PhantomData,
            foreign_key,
        }
    }

    pub async fn get(&self, model: &Parent) -> Result<Option<Child>, DatabaseError> {
        let pool = get_pool()?;
        let query = format!(
            "SELECT * FROM {} WHERE {} = ? LIMIT 1",
            Child::table_name(),
            self.foreign_key
        );
        
        let result = sqlx::query_as::<sqlx::Sqlite, Child>(&query)
            .bind(model.id())
            .fetch_optional(pool.as_ref())
            .await?;
            
        Ok(result)
    }
}

#[async_trait]
pub trait Model: for<'r> FromRow<'r, SqliteRow> + Serialize + DeserializeOwned + Send + Sync + Sized + Unpin {
    /// Get the table name for the model
    fn table_name() -> &'static str;

    /// Get the primary key name (defaults to "id")
    fn primary_key() -> &'static str {
        "id"
    }

    /// Get the model's ID
    fn id(&self) -> i64;

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
        
        let result = sqlx::query_as::<sqlx::Sqlite, Self>(&query)
            .bind(id)
            .fetch_optional(pool.as_ref())
            .await?;
            
        Ok(result)
    }

    /// Get all records
    async fn all() -> Result<Vec<Self>, DatabaseError> {
        let pool = get_pool()?;
        let query = format!("SELECT * FROM {}", Self::table_name());
        
        let results = sqlx::query_as::<sqlx::Sqlite, Self>(&query)
            .fetch_all(pool.as_ref())
            .await?;
            
        Ok(results)
    }

    /// Create a new record
    async fn create(model: Self) -> Result<Self, DatabaseError> {
        println!("Creating new record...");
        let pool = get_pool()?;
        println!("Got database pool successfully");
        
        // Convert the model to a JSON Value for field extraction
        let data = serde_json::to_value(&model)?;
        let obj = data.as_object().unwrap();
        
        // Filter out the id field since it's auto-generated
        let columns: Vec<String> = obj.keys()
            .filter(|&k| k != "id")
            .cloned()
            .collect();
            
        let placeholders: Vec<String> = (1..=columns.len()).map(|_| "?".to_string()).collect();

        // Create the insert query
        let insert_query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            Self::table_name(),
            columns.join(", "),
            placeholders.join(", ")
        );

        println!("Executing query: {}", insert_query);
        println!("With values: {:?}", obj);

        // Start a transaction
        let mut tx = pool.begin().await?;

        // Build and execute the insert query
        let mut query_builder = sqlx::query::<sqlx::Sqlite>(&insert_query);
        for column in &columns {
            if let Some(value) = obj.get(column) {
                match value {
                    Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            query_builder = query_builder.bind(i);
                        } else if let Some(f) = n.as_f64() {
                            query_builder = query_builder.bind(f);
                        }
                    },
                    Value::String(s) => {
                        // Get the raw string value without JSON escaping
                        let raw_string = s.as_str();
                        query_builder = query_builder.bind(raw_string);
                    },
                    Value::Bool(b) => query_builder = query_builder.bind(b),
                    Value::Null => query_builder = query_builder.bind(None::<String>),
                    _ => return Err(DatabaseError::Other(format!("Unsupported value type for column {}: {:?}", column, value))),
                }
            }
        }

        // Execute the insert
        let result = query_builder.execute(&mut *tx).await?;

        // Get the ID of the inserted row
        let id: i64 = result.last_insert_rowid();


        // Commit the transaction
        tx.commit().await?;

        // Log the result of the insert
        println!("Insert result: {:?}", result.last_insert_rowid());
        println!("Insert result: {:?}", result.rows_affected());
       
        

        // Fetch the created record
        let row = sqlx::query_as::<sqlx::Sqlite, Self>(&format!(
            "SELECT * FROM {} WHERE {} = ?",
            Self::table_name(),
            Self::primary_key()
        ))
        .bind(id)
        .fetch_one(pool.as_ref())
        .await?;

        Ok(row)
    }

    /// Get a field value by name (used for relationships)
    fn get_field_value(&self, field: &str) -> Result<i64, DatabaseError> {
        let value = serde_json::to_value(self)?;
        value.get(field)
            .and_then(|v| v.as_i64())
            .ok_or_else(|| DatabaseError::Other(format!("Field {} not found or invalid type", field)))
    }
} 