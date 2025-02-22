use std::time::Duration;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;
use once_cell::sync::OnceCell;

pub mod drivers;
pub mod config;

static CACHE_STORE: OnceCell<Arc<RwLock<Box<dyn CacheStore + Send + Sync>>>> = OnceCell::new();

#[async_trait]
pub trait CacheStore {
    async fn get(&self, key: &str) -> Option<Value>;
    async fn put(&self, key: &str, value: Value, ttl: Option<Duration>) -> bool;
    async fn forget(&self, key: &str) -> bool;
    async fn flush(&self) -> bool;
    async fn has(&self, key: &str) -> bool;
    async fn increment(&self, key: &str, value: i64) -> i64;
    async fn decrement(&self, key: &str, value: i64) -> i64;
}

/// A Laravel-like Cache facade for easy caching operations
pub struct Cache;

impl Cache {
    /// Get the underlying cache store
    pub fn store() -> Arc<RwLock<Box<dyn CacheStore + Send + Sync>>> {
        Arc::clone(CACHE_STORE.get().expect("Cache store not initialized"))
    }

    /// Retrieve an item from the cache
    pub async fn get<T: DeserializeOwned>(key: &str) -> Option<T> {
        let store = Self::store();
        let store = store.read().await;
        store.get(key).await.and_then(|v| serde_json::from_value(v).ok())
    }

    /// Store an item in the cache for a given number of seconds
    pub async fn put<T: Serialize>(key: &str, value: T, ttl: Duration) -> bool {
        let store = Self::store();
        let store = store.read().await;
        if let Ok(value) = serde_json::to_value(value) {
            store.put(key, value, Some(ttl)).await
        } else {
            false
        }
    }

    /// Store an item in the cache forever
    pub async fn forever<T: Serialize>(key: &str, value: T) -> bool {
        let store = Self::store();
        let store = store.read().await;
        if let Ok(value) = serde_json::to_value(value) {
            store.put(key, value, None).await
        } else {
            false
        }
    }

    /// Remove an item from the cache
    pub async fn forget(key: &str) -> bool {
        let store = Self::store();
        let store = store.read().await;
        store.forget(key).await
    }

    /// Remove all items from the cache
    pub async fn flush() -> bool {
        let store = Self::store();
        let store = store.read().await;
        store.flush().await
    }

    /// Determine if an item exists in the cache
    pub async fn has(key: &str) -> bool {
        let store = Self::store();
        let store = store.read().await;
        store.has(key).await
    }

    /// Increment the value of an item in the cache
    pub async fn increment(key: &str, value: i64) -> i64 {
        let store = Self::store();
        let store = store.read().await;
        store.increment(key, value).await
    }

    /// Decrement the value of an item in the cache
    pub async fn decrement(key: &str, value: i64) -> i64 {
        let store = Self::store();
        let store = store.read().await;
        store.decrement(key, value).await
    }

    /// Get an item from the cache, or store the default value forever
    pub async fn remember<T, F>(key: &str, ttl: Duration, callback: F) -> Option<T>
    where
        T: DeserializeOwned + Serialize,
        F: FnOnce() -> T + Send + Sync,
    {
        if let Some(value) = Self::get(key).await {
            return Some(value);
        }

        let value = callback();
        if Self::put(key, &value, ttl).await {
            Some(value)
        } else {
            None
        }
    }

    /// Get an item from the cache, or store the default value forever
    pub async fn remember_forever<T, F>(key: &str, callback: F) -> Option<T>
    where
        T: DeserializeOwned + Serialize,
        F: FnOnce() -> T + Send + Sync,
    {
        if let Some(value) = Self::get(key).await {
            return Some(value);
        }

        let value = callback();
        if Self::forever(key, &value).await {
            Some(value)
        } else {
            None
        }
    }

    /// Get an item from the cache or store the default value
    pub async fn sear<T, F>(key: &str, callback: F) -> Option<T>
    where
        T: DeserializeOwned + Serialize,
        F: FnOnce() -> T + Send + Sync,
    {
        Self::remember_forever(key, callback).await
    }

    /// Pull an item from the cache and delete it
    pub async fn pull<T: DeserializeOwned>(key: &str) -> Option<T> {
        let value = Self::get(key).await;
        if value.is_some() {
            Self::forget(key).await;
        }
        value
    }

    /// Store an item in the cache if the key doesn't exist
    pub async fn add<T: Serialize>(key: &str, value: T, ttl: Duration) -> bool {
        if Self::has(key).await {
            false
        } else {
            Self::put(key, value, ttl).await
        }
    }
} 