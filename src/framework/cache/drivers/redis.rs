use crate::framework::cache::CacheStore;
use async_trait::async_trait;
use redis::{AsyncCommands, Client, RedisError};
use serde_json::Value;
use std::time::Duration;

pub struct RedisStore {
    client: Client,
}

impl RedisStore {
    pub fn new(url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(url)?;
        Ok(Self { client })
    }
}

#[async_trait]
impl CacheStore for RedisStore {
    async fn get(&self, key: &str) -> Option<Value> {
        let mut conn = self.client.get_async_connection().await.ok()?;
        let value: Option<String> = conn.get(key).await.ok()?;
        value.and_then(|v| serde_json::from_str(&v).ok())
    }

    async fn put(&self, key: &str, value: Value, ttl: Option<Duration>) -> bool {
        let mut conn = match self.client.get_async_connection().await {
            Ok(conn) => conn,
            Err(_) => return false,
        };

        let value = match serde_json::to_string(&value) {
            Ok(v) => v,
            Err(_) => return false,
        };

        let set_result: Result<(), RedisError> = if let Some(ttl) = ttl {
            conn.set_ex(key, value, ttl.as_secs() as u64).await
        } else {
            conn.set(key, value).await
        };

        set_result.is_ok()
    }

    async fn forget(&self, key: &str) -> bool {
        if let Ok(mut conn) = self.client.get_async_connection().await {
            conn.del(key).await.unwrap_or(false)
        } else {
            false
        }
    }

    async fn flush(&self) -> bool {
        if let Ok(mut conn) = self.client.get_async_connection().await {
            let result: Result<(), RedisError> = redis::cmd("FLUSHDB")
                .query_async(&mut conn)
                .await;
            result.is_ok()
        } else {
            false
        }
    }

    async fn has(&self, key: &str) -> bool {
        if let Ok(mut conn) = self.client.get_async_connection().await {
            conn.exists(key).await.unwrap_or(false)
        } else {
            false
        }
    }

    async fn increment(&self, key: &str, value: i64) -> i64 {
        if let Ok(mut conn) = self.client.get_async_connection().await {
            conn.incr(key, value).await.unwrap_or(0)
        } else {
            0
        }
    }

    async fn decrement(&self, key: &str, value: i64) -> i64 {
        if let Ok(mut conn) = self.client.get_async_connection().await {
            conn.decr(key, value).await.unwrap_or(0)
        } else {
            0
        }
    }
} 