use crate::framework::{
    middleware::{
        Middleware,
        MiddlewareStack,
        presets::{Cors, TrimStrings}
    },
    views::{Metadata, set_global_metadata},
    inertia::InertiaConfig,
    database,
    cache::config::{CacheConfig, CacheDriver, init_cache},
    init_storage,
    StorageConfig,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;

/// Global application state
static APP: Lazy<Arc<RwLock<Application>>> = Lazy::new(|| {
    Arc::new(RwLock::new(Application::new()))
});

/// Application configuration
#[derive(Clone)]
pub struct Application {
    /// Global middleware stack
    middleware_stack: MiddlewareStack,
    /// Middleware groups
    middleware_groups: Vec<(String, Vec<Middleware>)>,
    /// Global metadata
    metadata: Option<Metadata>,
    /// Inertia configuration
    inertia_config: Option<InertiaConfig>,
}

impl Application {
    pub fn new() -> Self {
        Self {
            middleware_stack: MiddlewareStack::new(),
            middleware_groups: Vec::new(),
            metadata: None,
            inertia_config: None,
        }
    }

    /// Get the global application instance
    pub async fn instance() -> Arc<RwLock<Self>> {
        Arc::clone(&APP)
    }

    /// Configure global middleware
    pub async fn middleware<F>(&mut self, configure: F)
    where
        F: FnOnce(&mut MiddlewareStack),
    {
        configure(&mut self.middleware_stack);
    }

    /// Configure middleware groups
    pub async fn middleware_groups<F>(&mut self, configure: F)
    where
        F: FnOnce(&mut Vec<(String, Vec<Middleware>)>),
    {
        configure(&mut self.middleware_groups);
    }

    /// Configure global metadata
    pub async fn metadata<F>(&mut self, configure: F)
    where
        F: FnOnce() -> Metadata,
    {
        self.metadata = Some(configure());
        if let Some(metadata) = &self.metadata {
            set_global_metadata(metadata.clone());
        }
    }

    /// Configure Inertia
    pub async fn inertia<F>(&mut self, configure: F)
    where
        F: FnOnce() -> InertiaConfig,
    {
        self.inertia_config = Some(configure());
    }

    /// Get the global middleware stack
    pub fn middleware_stack(&self) -> MiddlewareStack {
        self.middleware_stack.clone()
    }

    /// Get the middleware groups
    pub fn groups(&self) -> Vec<(String, Vec<Middleware>)> {
        self.middleware_groups.clone()
    }

    /// Get the Inertia configuration
    pub fn inertia_config(&self) -> Option<InertiaConfig> {
        self.inertia_config.clone()
    }
}

/// Initialize the application with the given configuration
pub async fn bootstrap() -> Result<DatabaseConnection, String> {
    println!("Starting bootstrap process...");
    
    // Initialize the database connection
    println!("Initializing database connection...");
    let db = crate::framework::database::init()
        .await
        .map_err(|e| format!("Failed to initialize database: {}", e))?;
    println!("Database connection initialized successfully");

    // Load the cache configuration
    println!("Loading cache configuration...");
    let cache_config = CacheConfig::default();
    println!("Cache configuration loaded successfully");

    // Initialize the cache with the database connection
    println!("Initializing cache...");
    init_cache(cache_config, (*db).clone()).await
        .map_err(|e| format!("Failed to initialize cache: {}", e))?;
    println!("Cache initialized successfully");

    // Load the storage configuration
    println!("Loading storage configuration...");
    let storage_config = StorageConfig::default();
    println!("Storage configuration loaded successfully");

    // Initialize the storage system
    println!("Initializing storage system...");
    init_storage(storage_config).await
        .map_err(|e| format!("Failed to initialize storage: {}", e))?;
    println!("Storage system initialized successfully");

    Ok((*db).clone())
}

/// Get the application's middleware stack
pub async fn middleware_stack() -> MiddlewareStack {
    let app = Application::instance().await;
    let app = app.read().await;
    app.middleware_stack()
}

/// Get a middleware group by name
pub async fn middleware_group(name: &str) -> Option<Vec<Middleware>> {
    let app = Application::instance().await;
    let app = app.read().await;
    app.groups()
        .iter()
        .find(|(group_name, _)| group_name == name)
        .map(|(_, middlewares)| middlewares.clone())
} 