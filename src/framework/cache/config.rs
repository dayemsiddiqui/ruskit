use std::time::Duration;
use sea_orm::DatabaseConnection;
use crate::framework::cache::{CacheStore, CACHE_STORE};
use crate::framework::cache::drivers::{DatabaseStore, RedisStore};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub enum CacheDriver {
    Database,
    Redis,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub driver: CacheDriver,
    pub redis_url: Option<String>,
    pub default_ttl: Option<Duration>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            driver: CacheDriver::Database,
            redis_url: None,
            default_ttl: Some(Duration::from_secs(3600)), // 1 hour
        }
    }
}

pub async fn init_cache(config: CacheConfig, db: DatabaseConnection) -> Result<(), String> {
    // If cache store is already initialized, return early
    if CACHE_STORE.get().is_some() {
        return Ok(());
    }

    let store: Box<dyn CacheStore + Send + Sync> = match config.driver {
        CacheDriver::Database => Box::new(DatabaseStore::new(db)),
        CacheDriver::Redis => {
            let redis_url = config.redis_url.ok_or("Redis URL not configured")?;
            Box::new(RedisStore::new(&redis_url).map_err(|e| e.to_string())?)
        }
    };

    CACHE_STORE
        .set(Arc::new(RwLock::new(store)))
        .map_err(|_| "Failed to initialize cache store".to_string())
} 