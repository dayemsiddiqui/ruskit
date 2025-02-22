use crate::cli::error::CliError;
use crate::framework::database::{
    initialize,
    config::DatabaseConfig,
};

pub async fn initialize_database() -> Result<(), CliError> {
    println!("Initializing database...");
    let db_config = DatabaseConfig::from_env();
    initialize(Some(db_config))
        .await
        .map_err(|e| CliError::DatabaseError(e.to_string()))?;
    
    println!("Database initialization completed successfully");
    Ok(())
} 