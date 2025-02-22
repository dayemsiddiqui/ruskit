use sea_orm::*;
use std::time::Duration;
use once_cell::sync::OnceCell;

static DB: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn init() -> Result<DatabaseConnection, DbErr> {
    if let Some(conn) = DB.get() {
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

    match DB.set(connection.clone()) {
        Ok(_) => Ok(connection),
        Err(_) => {
            if let Some(conn) = DB.get() {
                Ok(conn.clone())
            } else {
                Err(DbErr::Custom("Failed to initialize database connection".to_string()))
            }
        }
    }
}

pub fn get_connection() -> &'static DatabaseConnection {
    DB.get()
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