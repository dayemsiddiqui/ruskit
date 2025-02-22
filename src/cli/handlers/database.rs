use crate::cli::error::CliError;
use crate::framework::database::{
    initialize,
    config::DatabaseConfig,
};
use crate::app;

pub async fn generate_entities() -> Result<(), CliError> {
    use crate::framework::database::schema;
    
    println!("Initializing application...");
    app::initialize();
    
    println!("Initializing database...");
    let db_config = DatabaseConfig::from_env();
    let pool = initialize(Some(db_config))
        .await
        .map_err(|e| CliError::DatabaseError(e.to_string()))?;
    
    println!("Generating entities from database schema...");
    schema::generate_all_entities(&pool)
        .await
        .map_err(|e| CliError::DatabaseError(e.to_string()))?;
    
    println!("Entity generation completed successfully");
    Ok(())
} 