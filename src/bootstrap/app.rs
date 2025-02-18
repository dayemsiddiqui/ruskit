use crate::framework::{
    middleware::{
        Middleware,
        MiddlewareStack,
        presets::{Cors, TrimStrings}
    },
    views::{Metadata, set_global_metadata},
    database::{self, migration::MigrationManager},
};
use std::sync::Arc;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;
use crate::app::models;
use std::path::Path;
use std::fs;
use sqlx::sqlite::SqlitePool;

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
}

impl Application {
    pub fn new() -> Self {
        Self {
            middleware_stack: MiddlewareStack::new(),
            middleware_groups: Vec::new(),
            metadata: None,
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

    /// Get the global middleware stack
    pub fn middleware_stack(&self) -> MiddlewareStack {
        self.middleware_stack.clone()
    }

    /// Get the middleware groups
    pub fn groups(&self) -> Vec<(String, Vec<Middleware>)> {
        self.middleware_groups.clone()
    }
}

/// Initialize the application
pub async fn bootstrap() -> Result<Application, Box<dyn std::error::Error>> {
    // Register all models
    models::register_models();

    // Initialize database
    println!("Initializing database...");
    let db_config = database::config::DatabaseConfig::from_env();
    println!("Initializing database at path: {}", db_config.database_path());
    
    // Create database directory if it doesn't exist
    if let Some(parent) = Path::new(&db_config.database_path()).parent() {
        println!("Creating database directory: {}", parent.display());
        fs::create_dir_all(parent)?;
    }
    
    println!("Connecting to database with URL: {}", db_config.connection_url());
    let pool = SqlitePool::connect(&db_config.connection_url()).await?;
    println!("Successfully connected to database");
    
    // Enable foreign key constraints
    println!("Enabling foreign key constraints");
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await?;
    
    // Store the pool globally
    *database::POOL.lock().unwrap() = Some(Arc::new(pool));
    println!("Database pool initialized successfully");


    let app = Application::instance().await;
    let mut app = app.write().await;

    // Configure global middleware
    app.middleware(|stack| {
        stack.add(Middleware::Cors(Cors::new("*")));
        stack.add(Middleware::TrimStrings(TrimStrings::new()));
    }).await;

    // Configure middleware groups
    app.middleware_groups(|groups| {
        groups.push(("api".to_string(), vec![
            Middleware::Cors(Cors::new("*")),
            Middleware::TrimStrings(TrimStrings::new()),
        ]));
    }).await;

    // Configure global metadata
    app.metadata(|| {
        Metadata::new("Ruskit")
            .with_description("A modern web framework for Rust")
            .with_keywords("rust, web, framework")
            .with_author("Your Name")
            .with_og_title("Ruskit")
            .with_og_description("A modern web framework for Rust")
            .with_og_image("https://example.com/og-image.jpg")
    }).await;

    let app_clone = app.clone();
    Ok(app_clone)
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