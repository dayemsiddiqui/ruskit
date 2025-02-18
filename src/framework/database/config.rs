use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub default: String,
    pub connections: Connections,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connections {
    pub sqlite: SqliteConnection,
    // Add more connection types here as needed
    // pub mysql: MySqlConnection,
    // pub postgres: PostgresConnection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqliteConnection {
    pub database: String,
    pub foreign_key_constraints: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        // Get the current working directory
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let default_db_path = current_dir.join("database").join("database.sqlite");
        
        Self {
            default: env::var("DB_CONNECTION").unwrap_or_else(|_| "sqlite".to_string()),
            connections: Connections {
                sqlite: SqliteConnection {
                    database: env::var("DB_DATABASE")
                        .unwrap_or_else(|_| default_db_path.to_string_lossy().to_string()),
                    foreign_key_constraints: true,
                },
            },
        }
    }
}

impl DatabaseConfig {
    pub fn connection_url(&self) -> String {
        match self.default.as_str() {
            "sqlite" => {
                // For SQLite, we need three slashes for absolute paths
                if self.connections.sqlite.database.starts_with('/') {
                    format!("sqlite:///{}", self.connections.sqlite.database)
                } else {
                    format!("sqlite://{}", self.connections.sqlite.database)
                }
            }
            // Add more connection types here
            _ => panic!("Unsupported database connection type"),
        }
    }

    pub fn from_env() -> Self {
        Self::default()
    }
}

// Global configuration instance
static mut CONFIG: Option<DatabaseConfig> = None;

pub fn set_config(config: DatabaseConfig) {
    unsafe {
        CONFIG = Some(config);
    }
}

pub fn get_config() -> DatabaseConfig {
    unsafe {
        CONFIG.clone().unwrap_or_else(DatabaseConfig::default)
    }
} 