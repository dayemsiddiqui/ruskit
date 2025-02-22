use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{FromRow, sqlite::SqliteRow};
use crate::framework::database::{get_pool, DatabaseError};
use std::sync::Mutex;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::fs;
use std::path::Path;
use serde_json::Value;
use std::marker::PhantomData;
use validator::ValidationError;
use regex::Regex;
use paste;

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
                
                println!("Discovered model: {}", full_type_name);
            }
        }
    }
    
    Ok(())
}

pub struct BelongsTo<Parent: Model> {
    parent_type: PhantomData<Parent>,
    foreign_key: String,
}

pub struct HasMany<Child: Model> {
    child_type: PhantomData<Child>,
    foreign_key: String,
}

pub struct HasOne<Parent: Model, Child: Model> {
    parent_type: PhantomData<Parent>,
    child_type: PhantomData<Child>,
    foreign_key: &'static str,
}

impl<Parent: Model> BelongsTo<Parent> {
    pub fn new<Child: Model>() -> Self {
        // Get the parent table name and remove trailing 's' if present
        let parent_table = Parent::table_name();
        let singular = if parent_table.ends_with('s') {
            &parent_table[..parent_table.len() - 1]
        } else {
            parent_table
        };
        
        // Construct foreign key (e.g., "user_id" from "users")
        let foreign_key = format!("{}_id", singular);
        
        Self {
            parent_type: PhantomData,
            foreign_key,
        }
    }

    pub fn with_key(foreign_key: impl Into<String>) -> Self {
        Self {
            parent_type: PhantomData,
            foreign_key: foreign_key.into(),
        }
    }

    pub async fn get(&self, model: &impl Model) -> Result<Option<Parent>, DatabaseError> {
        let foreign_key_value = model.get_field_value(&self.foreign_key)?;
        Parent::find(foreign_key_value).await
    }
}

impl<Child: Model> HasMany<Child> {
    pub fn new<Parent: Model>() -> Self {
        // Get the parent table name and remove trailing 's' if present
        let parent_table = Parent::table_name();
        let singular = if parent_table.ends_with('s') {
            &parent_table[..parent_table.len() - 1]
        } else {
            parent_table
        };
        
        // Construct foreign key (e.g., "user_id" from "users")
        let foreign_key = format!("{}_id", singular);
        
        Self {
            child_type: PhantomData,
            foreign_key,
        }
    }

    pub fn with_key(foreign_key: impl Into<String>) -> Self {
        Self {
            child_type: PhantomData,
            foreign_key: foreign_key.into(),
        }
    }

    pub async fn get(&self, model: &impl Model) -> Result<Vec<Child>, DatabaseError> {
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

    pub async fn create(&self, mut model: Child) -> Result<Child, DatabaseError> {
        let pool = get_pool()?;
        let query = format!(
            "UPDATE {} SET {} = ? WHERE id = ?",
            Child::table_name(),
            self.foreign_key
        );
        
        let created = Child::create(model).await?;
        sqlx::query(&query)
            .bind(created.id())
            .execute(pool.as_ref())
            .await?;
            
        Ok(created)
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

// Validation rules
#[derive(Clone)]
pub enum Rule {
    Required,
    Email,
    MinLength(usize),
    MaxLength(usize),
    Regex(String),
}

pub struct Rules(Vec<Rule>);

impl Rules {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn required(mut self) -> Self {
        self.0.push(Rule::Required);
        self
    }

    pub fn email(mut self) -> Self {
        self.0.push(Rule::Email);
        self
    }

    pub fn min(mut self, length: usize) -> Self {
        self.0.push(Rule::MinLength(length));
        self
    }

    pub fn max(mut self, length: usize) -> Self {
        self.0.push(Rule::MaxLength(length));
        self
    }

    pub fn regex(mut self, pattern: &str) -> Self {
        self.0.push(Rule::Regex(pattern.to_string()));
        self
    }
}

impl Rule {
    fn validate(&self, field: &str, value: &str) -> Result<(), ValidationError> {
        match self {
            Rule::Required => {
                if value.trim().is_empty() {
                    return Err(ValidationError::new("required field"));
                }
            }
            Rule::Email => {
                let email_regex = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
                if !email_regex.is_match(value) {
                    return Err(ValidationError::new("invalid email format"));
                }
            }
            Rule::MinLength(min) => {
                if value.len() < *min {
                    return Err(ValidationError::new("too short"));
                }
            }
            Rule::MaxLength(max) => {
                if value.len() > *max {
                    return Err(ValidationError::new("too long"));
                }
            }
            Rule::Regex(pattern) => {
                let regex = Regex::new(pattern).unwrap();
                if !regex.is_match(value) {
                    return Err(ValidationError::new("invalid format"));
                }
            }
        }
        Ok(())
    }
}

pub struct Field<T> {
    name: &'static str,
    _type: PhantomData<T>,
}

impl<T> Field<T> {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            _type: PhantomData,
        }
    }
}

pub trait Validate {
    fn validate(&self, rules: Rules) -> Result<(), ValidationError>;
}

impl Validate for String {
    fn validate(&self, rules: Rules) -> Result<(), ValidationError> {
        for rule in rules.0 {
            rule.validate(self, self.as_str())?;
        }
        Ok(())
    }
}

/// Trait for custom validation rules
pub trait ValidationRules {
    fn validate_rules(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

/// Trait for defining model-specific validation rules
pub trait ModelValidation: Sized {
    type Fields;
    
    fn fields() -> Self::Fields;
    
    fn validate(&self) -> Result<(), ValidationError> where Self: ValidationRules {
        self.validate_rules()
    }
}

#[async_trait]
pub trait Model: for<'r> FromRow<'r, SqliteRow> + Serialize + DeserializeOwned + Send + Sync + Sized + Unpin + ModelValidation + ValidationRules {
    /// Get the table name for the model
    fn table_name() -> &'static str;

    /// Get the primary key name (defaults to "id")
    fn primary_key() -> &'static str {
        "id"
    }

    /// Get the model's ID
    fn id(&self) -> i64;

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

    /// Create a new record with validation
    async fn create_validated(model: Self) -> Result<Self, DatabaseError> {
        model.validate().map_err(|e| DatabaseError::Other(e.to_string()))?;
        Self::create(model).await
    }
}

#[macro_export]
macro_rules! define_fields {
    ($name:ident { $($field:ident: $type:ty),* $(,)? }) => {
        pub struct $name {
            $(pub $field: Field<$type>,)*
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    $($field: Field::new(stringify!($field)),)*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! generate_validation_fields {
    ($model:ident) => {
        paste::paste! {
            pub struct [<$model Fields>] {
                $(pub $field: Field<$type>,)*
            }

            impl [<$model Fields>] {
                pub fn new() -> Self {
                    Self {
                        $(pub $field: Field::new(stringify!($field)),)*
                    }
                }
            }

            impl ModelValidation for $model {
                type Fields = [<$model Fields>];

                fn fields() -> Self::Fields {
                    [<$model Fields>]::new()
                }
            }
        }
    };
} 