use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{FromRow, Row, sqlite::SqliteRow};
use crate::framework::database::{get_pool, DatabaseError};
use crate::framework::database::migration::Migration;

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
    async fn create(attributes: impl Serialize + Send + 'static) -> Result<Self, DatabaseError> {
        let pool = get_pool()?;
        let json = serde_json::to_value(attributes)?;
        
        if let serde_json::Value::Object(map) = json {
            let columns: Vec<String> = map.keys().map(|k| k.to_string()).collect();
            let values: Vec<_> = (0..columns.len()).map(|_| "?").collect();
            
            let query = format!(
                "INSERT INTO {} ({}) VALUES ({})",
                Self::table_name(),
                columns.join(", "),
                values.join(", ")
            );
            
            let mut query_builder = sqlx::query(&query);
            for value in map.values() {
                query_builder = query_builder.bind(value);
            }
            
            query_builder.execute(pool.as_ref()).await?;
            
            // Get the last inserted record
            let last_id = sqlx::query("SELECT last_insert_rowid()")
                .fetch_one(pool.as_ref())
                .await?
                .try_get::<i64, _>(0)?;
                
            Self::find(last_id).await?.ok_or(DatabaseError::ConnectionError(
                sqlx::Error::RowNotFound
            ))
        } else {
            Err(DatabaseError::ConnectionError(
                sqlx::Error::Configuration("Invalid attributes".into())
            ))
        }
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(error: serde_json::Error) -> Self {
        DatabaseError::ConnectionError(sqlx::Error::Configuration(error.to_string().into()))
    }
} 