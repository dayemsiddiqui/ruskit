pub mod cli;
pub mod web;
pub mod bootstrap;
pub mod framework;
pub mod app;
pub mod config;
pub mod routes;

// Re-export framework types
pub use framework::middleware::{
    Middleware,
    RouteMiddlewareExt,
    RouterMiddlewareExt,
    WithMiddleware,
    presets,
};

// Re-export bootstrap functions
pub use bootstrap::app::{
    bootstrap,
    middleware_stack,
    middleware_group,
};

pub async fn setup() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();
    
    // Initialize database connection
    framework::database::init().await?;
    
    Ok(())
}

// Re-export commonly used items
pub use framework::prelude::*; 