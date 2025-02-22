use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::framework::storage::drivers::LocalDriver;
use crate::framework::storage::STORAGE_DRIVER;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    #[serde(default = "default_driver")]
    pub default: String,
    pub disks: Disks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Disks {
    #[serde(default = "default_local_config")]
    pub local: LocalDiskConfig,
    // Uncomment when implementing S3 and R2 drivers
    // pub s3: S3DiskConfig,
    // pub r2: R2DiskConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalDiskConfig {
    #[serde(default = "default_storage_path")]
    pub root: PathBuf,
    #[serde(default = "default_storage_url")]
    pub url: String,
}

// Example S3 configuration for future implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3DiskConfig {
    pub key: String,
    pub secret: String,
    pub region: String,
    pub bucket: String,
    pub url: Option<String>,
    pub endpoint: Option<String>,
}

// Example R2 configuration for future implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R2DiskConfig {
    pub account_id: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub bucket: String,
    pub url: Option<String>,
}

fn default_driver() -> String {
    env::var("STORAGE_DRIVER").unwrap_or_else(|_| "local".to_string())
}

fn default_storage_path() -> PathBuf {
    PathBuf::from(env::var("STORAGE_PATH").unwrap_or_else(|_| "storage".to_string()))
}

fn default_storage_url() -> String {
    env::var("STORAGE_URL").unwrap_or_else(|_| "http://localhost:3000/storage".to_string())
}

fn default_local_config() -> LocalDiskConfig {
    LocalDiskConfig {
        root: default_storage_path(),
        url: default_storage_url(),
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            default: default_driver(),
            disks: Disks {
                local: default_local_config(),
            },
        }
    }
}

/// Initialize the storage system with the provided configuration
pub async fn init_storage(config: StorageConfig) -> Result<(), Box<dyn std::error::Error>> {
    let driver: Box<dyn crate::framework::storage::StorageDriver + Send + Sync> = match config.default.as_str() {
        "local" => {
            Box::new(LocalDriver::new(
                config.disks.local.root,
                &config.disks.local.url,
            ).await?)
        }
        // Add other drivers here when implemented
        _ => return Err(format!("Unsupported storage driver: {}", config.default).into()),
    };

    STORAGE_DRIVER
        .set(Arc::new(RwLock::new(driver)))
        .map_err(|_| "Failed to initialize storage driver".into())
} 