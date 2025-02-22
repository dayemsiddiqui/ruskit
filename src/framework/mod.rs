pub mod inertia;
pub mod middleware;
pub mod routing;
pub mod typescript;
pub mod views;
pub mod prelude;
pub mod database;
pub mod cache;
pub mod storage;
pub mod bootstrap;
pub mod cli;
pub mod config;
pub mod app;
pub mod schedule;
pub mod queue;

// Re-export framework types
pub use middleware::{
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

// Re-export commonly used items
pub use prelude::*;

// Re-export storage and cache functionality
pub use storage::Storage;
pub use storage::config::{StorageConfig, LocalDiskConfig, init_storage};
pub use cache::Cache;
pub use cache::config::{CacheConfig, CacheDriver, init_cache};

// Re-export config
pub use config::AppConfig;

// Re-export app functions
pub use app::{run, generate_typescript_types};

pub use queue::Queue;

pub async fn setup() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();
    
    // Initialize database connection
    database::init().await?;
    
    Ok(())
}

pub use views::*;
pub use typescript::export_all_types; 