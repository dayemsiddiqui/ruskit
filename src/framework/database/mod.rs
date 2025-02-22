use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::sync::OnceCell;

static DATABASE: OnceLock<Arc<DatabaseConnection>> = OnceLock::new();

/// A Laravel-like DB facade for easy database access
pub struct DB;

impl DB {
    /// Get the database connection
    pub fn connection() -> &'static DatabaseConnection {
        &**DATABASE.get()
            .expect("Database connection not initialized")
    }

    /// Initialize the database connection with the given options
    pub async fn init(mut options: ConnectOptions) -> Result<(), Box<dyn std::error::Error>> {
        // Set default options if not set
        options
            .max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true);

        let connection = Database::connect(options).await?;
        let connection = Arc::new(connection);

        // If we fail to set the connection, it means another thread already initialized it
        // Just return Ok since we have a valid connection
        let _ = DATABASE.set(connection);
        Ok(())
    }

    /// Get the database connection as an Arc
    pub fn connection_arc() -> Arc<DatabaseConnection> {
        DATABASE.get()
            .expect("Database connection not initialized")
            .clone()
    }

    /// Get the database connection as a reference
    pub fn connection_ref() -> &'static DatabaseConnection {
        &**DATABASE.get()
            .expect("Database connection not initialized")
    }

    /// Get the database connection as a reference, or initialize it if not already initialized
    pub async fn get_or_init(options: ConnectOptions) -> Result<&'static DatabaseConnection, Box<dyn std::error::Error>> {
        if let Some(conn) = DATABASE.get() {
            Ok(&**conn)
        } else {
            Self::init(options).await?;
            Ok(Self::connection())
        }
    }

    /// Get the database connection as an Arc, or initialize it if not already initialized
    pub async fn get_or_init_arc(options: ConnectOptions) -> Result<Arc<DatabaseConnection>, Box<dyn std::error::Error>> {
        if let Some(conn) = DATABASE.get() {
            Ok(conn.clone())
        } else {
            Self::init(options).await?;
            Ok(Self::connection_arc())
        }
    }

    /// Initialize the database connection with a URL string
    pub async fn init_with_url(url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let options = ConnectOptions::new(url.to_string());
        Self::init(options).await
    }

    /// Get the database connection as a reference, or initialize it with a URL string if not already initialized
    pub async fn get_or_init_with_url(url: &str) -> Result<&'static DatabaseConnection, Box<dyn std::error::Error>> {
        if let Some(conn) = DATABASE.get() {
            Ok(&**conn)
        } else {
            Self::init_with_url(url).await?;
            Ok(Self::connection())
        }
    }
}

pub async fn init() -> Result<Arc<DatabaseConnection>, DbErr> {
    if let Some(conn) = DATABASE.get() {
        return Ok(conn.clone());
    }

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment");

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    let connection = Database::connect(opt).await?;
    let connection = Arc::new(connection);

    match DATABASE.set(connection.clone()) {
        Ok(_) => Ok(connection),
        Err(_) => {
            if let Some(conn) = DATABASE.get() {
                Ok(conn.clone())
            } else {
                Err(DbErr::Custom("Failed to initialize database connection".to_string()))
            }
        }
    }
}

pub fn get_connection() -> &'static DatabaseConnection {
    DATABASE.get()
        .expect("Database connection not initialized")
}

pub async fn setup_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(database_url.to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    Database::connect(opt).await
} 