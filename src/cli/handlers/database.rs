use crate::cli::error::CliError;
use crate::framework::database::{
    migration::MigrationManager,
    initialize,
    config::DatabaseConfig,
};
use crate::app;

pub async fn run_migrate() -> Result<(), CliError> {
    println!("Initializing application...");
    app::initialize();
    
    println!("Initializing database...");
    let db_config = DatabaseConfig::from_env();
    let _pool = initialize(Some(db_config))
        .await
        .map_err(|e| CliError::DatabaseError(e.to_string()))?;
    
    println!("Running migrations...");
    let manager = MigrationManager::new()
        .await
        .map_err(|e| CliError::MigrationError(e.to_string()))?;
    
    manager.run(manager.get_all_model_migrations())
        .await
        .map_err(|e| CliError::MigrationError(e.to_string()))?;
    
    println!("Migrations completed successfully");
    Ok(())
}

pub async fn run_fresh() -> Result<(), CliError> {
    println!("Initializing application...");
    app::initialize();
    
    println!("Initializing database...");
    let db_config = DatabaseConfig::from_env();
    let _pool = initialize(Some(db_config))
        .await
        .map_err(|e| CliError::DatabaseError(e.to_string()))?;
    
    let manager = MigrationManager::new()
        .await
        .map_err(|e| CliError::MigrationError(e.to_string()))?;
    
    println!("Dropping all tables...");
    manager.drop_all_tables()
        .await
        .map_err(|e| CliError::DatabaseError(e.to_string()))?;
    println!("All tables dropped successfully");
    
    println!("Running fresh migrations...");
    manager.run(manager.get_all_model_migrations())
        .await
        .map_err(|e| CliError::MigrationError(e.to_string()))?;
    println!("Fresh migrations completed successfully");
    
    Ok(())
}

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