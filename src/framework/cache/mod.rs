use std::time::Duration;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;
use once_cell::sync::OnceCell;
use std::future::Future;
use std::pin::Pin;

pub mod drivers;
pub mod config;

/// A trait to easily box futures for the cache system
pub trait BoxFuture<T>: Future<Output = T> + Send + 'static {
    fn boxed(self) -> Pin<Box<dyn Future<Output = T> + Send + 'static>>;
}

impl<F, T> BoxFuture<T> for F
where
    F: Future<Output = T> + Send + 'static,
{
    fn boxed(self) -> Pin<Box<dyn Future<Output = T> + Send + 'static>> {
        Box::pin(self)
    }
}

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

    /// Get an item from the cache, or store the default value with a TTL
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

    /// Get an item from the cache with stale-while-revalidate behavior.
    /// Returns the cached value (even if stale) while asynchronously revalidating it.
    pub async fn flexible<T, F, Fut>(key: &str, ttl: Duration, grace: Duration, callback: F) -> Option<T>
    where
        T: DeserializeOwned + Serialize + Clone + Send + 'static,
        F: FnOnce() -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = T> + Send + 'static,
    {
        let store = Self::store();
        let store_read = store.read().await;
        
        // Try to get the value from cache
        if let Some(value) = store_read.get(key).await {
            // If we have a value, spawn a background task to refresh it if it's stale
            let key = key.to_string();
            let callback = callback.clone();
            let store = store.clone();
            
            tokio::spawn(async move {
                // Check if the value is stale and needs revalidation
                if let Some(_) = store.read().await.get(&key).await {
                    // If we have a value, check if it's stale
                    let new_value = callback().await;
                    if let Ok(value) = serde_json::to_value(new_value) {
                        let _ = store.read().await.put(&key, value, Some(ttl)).await;
                    }
                }
            });

            // Return the current value (might be stale) immediately
            return serde_json::from_value(value).ok();
        }

        // If no value exists, generate it synchronously
        let value = callback().await;
        if let Ok(json_value) = serde_json::to_value(value.clone()) {
            let _ = store_read.put(key, json_value, Some(ttl + grace)).await;
        }
        Some(value)
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