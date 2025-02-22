use crate::framework::cache::CacheStore;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait, QueryFilter, ColumnTrait};
use serde_json::Value;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "cache")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub key: String,
    #[sea_orm(column_type = "Text")]
    pub value: String,
    pub expiration: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub struct DatabaseStore {
    db: DatabaseConnection,
}

impl DatabaseStore {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn get_expiration(ttl: Option<Duration>) -> Option<i64> {
        ttl.map(|duration| {
            SystemTime::now()
                .checked_add(duration)
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64
        })
    }
}

#[async_trait]
impl CacheStore for DatabaseStore {
    async fn get(&self, key: &str) -> Option<Value> {
        let cache = Entity::find()
            .filter(Column::Key.eq(key))
            .one(&self.db)
            .await
            .ok()?;

        if let Some(cache) = cache {
            if let Some(expiration) = cache.expiration {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;
                if now > expiration {
                    self.forget(key).await;
                    return None;
                }
            }
            serde_json::from_str(&cache.value).ok()
        } else {
            None
        }
    }

    async fn put(&self, key: &str, value: Value, ttl: Option<Duration>) -> bool {
        let value = match serde_json::to_string(&value) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let expiration = Self::get_expiration(ttl);

        let cache = ActiveModel {
            key: Set(key.to_string()),
            value: Set(value),
            expiration: Set(expiration),
        };

        cache.insert(&self.db).await.is_ok()
    }

    async fn forget(&self, key: &str) -> bool {
        Entity::delete_many()
            .filter(Column::Key.eq(key))
            .exec(&self.db)
            .await
            .map(|res| res.rows_affected > 0)
            .unwrap_or(false)
    }

    async fn flush(&self) -> bool {
        Entity::delete_many()
            .exec(&self.db)
            .await
            .map(|res| res.rows_affected > 0)
            .unwrap_or(false)
    }

    async fn has(&self, key: &str) -> bool {
        Entity::find()
            .filter(Column::Key.eq(key))
            .one(&self.db)
            .await
            .map(|res| res.is_some())
            .unwrap_or(false)
    }

    async fn increment(&self, key: &str, value: i64) -> i64 {
        if let Some(current) = self.get(key).await {
            if let Some(current) = current.as_i64() {
                let new_value = current + value;
                if self.put(key, Value::from(new_value), None).await {
                    return new_value;
                }
            }
        }
        0
    }

    async fn decrement(&self, key: &str, value: i64) -> i64 {
        self.increment(key, -value).await
    }
} 