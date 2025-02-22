use crate::framework::queue::{QueueDriver, QueuedJob};
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait, QueryFilter, ColumnTrait, QueryOrder};
use std::time::Duration;
use sea_orm::entity::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "jobs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub queue: String,
    #[sea_orm(column_type = "Text")]
    pub payload: String,
    pub attempts: u32,
    pub reserved_at: Option<DateTimeWithTimeZone>,
    pub available_at: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub struct DatabaseDriver {
    db: DatabaseConnection,
}

impl DatabaseDriver {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl QueueDriver for DatabaseDriver {
    async fn push(&self, queue: &str, payload: String, delay: Option<Duration>) -> Result<Uuid, Box<dyn std::error::Error>> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let available_at = if let Some(delay) = delay {
            now + chrono::Duration::from_std(delay)?
        } else {
            now
        };

        let model = ActiveModel {
            id: Set(id),
            queue: Set(queue.to_string()),
            payload: Set(payload),
            attempts: Set(0),
            reserved_at: Set(None),
            available_at: Set(available_at.into()),
            created_at: Set(now.into()),
        };
        model.insert(&self.db).await?;
        Ok(id)
    }

    async fn pop(&self, queue: &str) -> Option<QueuedJob> {
        let now = Utc::now();
        if let Ok(result) = Entity::find()
            .filter(Column::Queue.eq(queue))
            .filter(Column::ReservedAt.is_null())
            .filter(Column::AvailableAt.lte(now))
            .order_by_asc(Column::Id)
            .one(&self.db)
            .await {
            if let Some(job) = result {
                let mut job: ActiveModel = job.into();
                job.reserved_at = Set(Some(now.into()));
                if let Ok(job) = job.update(&self.db).await {
                    return Some(QueuedJob {
                        id: job.id,
                        queue: job.queue,
                        payload: job.payload,
                        attempts: job.attempts,
                        reserved_at: job.reserved_at.map(|dt| dt.into()),
                        available_at: job.available_at.into(),
                        created_at: job.created_at.into(),
                    });
                }
            }
        }
        None
    }

    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;
        Ok(())
    }

    async fn release(&self, id: Uuid, delay: Option<Duration>) -> Result<(), Box<dyn std::error::Error>> {
        let job = Entity::find_by_id(id)
            .one(&self.db)
            .await?;

        if let Some(job) = job {
            let mut job: ActiveModel = job.into();
            job.reserved_at = Set(None);
            job.available_at = Set(if let Some(delay) = delay {
                (Utc::now() + chrono::Duration::from_std(delay).unwrap()).into()
            } else {
                Utc::now().into()
            });
            job.attempts = Set(job.attempts.unwrap() + 1);
            job.update(&self.db).await?;
        }
        Ok(())
    }

    async fn size(&self, queue: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let count = Entity::find()
            .filter(Column::Queue.eq(queue))
            .count(&self.db)
            .await?;
        Ok(count.try_into()?)
    }

    async fn clear(&self, queue: &str) -> Result<(), Box<dyn std::error::Error>> {
        Entity::delete_many()
            .filter(Column::Queue.eq(queue))
            .exec(&self.db)
            .await?;
        Ok(())
    }
} 