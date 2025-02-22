use std::sync::Arc;
use tokio::sync::RwLock;
use once_cell::sync::OnceCell;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub mod drivers;
pub mod config;
pub mod worker;
pub mod job;

pub use job::Job;

static QUEUE_DRIVER: OnceCell<Arc<RwLock<Box<dyn QueueDriver + Send + Sync>>>> = OnceCell::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedJob {
    pub id: Uuid,
    pub queue: String,
    pub payload: String,
    pub attempts: u32,
    pub reserved_at: Option<DateTime<Utc>>,
    pub available_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[async_trait]
pub trait QueueDriver {
    /// Push a new job onto the queue
    async fn push(&self, queue: &str, payload: String, delay: Option<Duration>) -> Result<Uuid, Box<dyn std::error::Error>>;
    
    /// Get the next available job from the queue
    async fn pop(&self, queue: &str) -> Option<QueuedJob>;
    
    /// Delete a job from the queue
    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Release a job back onto the queue
    async fn release(&self, id: Uuid, delay: Option<Duration>) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Get the size of the queue
    async fn size(&self, queue: &str) -> Result<u64, Box<dyn std::error::Error>>;
    
    /// Clear all jobs from the queue
    async fn clear(&self, queue: &str) -> Result<(), Box<dyn std::error::Error>>;
}

/// A Laravel-like Queue facade for easy job queueing
pub struct Queue;

impl Queue {
    /// Get the queue driver instance
    pub fn driver() -> Arc<RwLock<Box<dyn QueueDriver + Send + Sync>>> {
        QUEUE_DRIVER
            .get()
            .expect("Queue driver not initialized")
            .clone()
    }

    /// Push a new job onto the queue
    pub async fn push(queue: &str, payload: String, delay: Option<Duration>) -> Result<Uuid, Box<dyn std::error::Error>> {
        let driver = Self::driver();
        let driver = driver.read().await;
        driver.push(queue, payload, delay).await
    }

    /// Get the next available job from the queue
    pub async fn pop(queue: &str) -> Option<QueuedJob> {
        let driver = Self::driver();
        let driver = driver.read().await;
        driver.pop(queue).await
    }

    /// Delete a job from the queue
    pub async fn delete(id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let driver = Self::driver();
        let driver = driver.read().await;
        driver.delete(id).await
    }

    /// Release a job back onto the queue
    pub async fn release(id: Uuid, delay: Option<Duration>) -> Result<(), Box<dyn std::error::Error>> {
        let driver = Self::driver();
        let driver = driver.read().await;
        driver.release(id, delay).await
    }

    /// Get the size of the queue
    pub async fn size(queue: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let driver = Self::driver();
        let driver = driver.read().await;
        driver.size(queue).await
    }

    /// Clear all jobs from the queue
    pub async fn clear(queue: &str) -> Result<(), Box<dyn std::error::Error>> {
        let driver = Self::driver();
        let driver = driver.read().await;
        driver.clear(queue).await
    }

    /// Dispatch a job to the queue
    pub async fn dispatch<T: Job>(job: T) -> Result<Uuid, Box<dyn std::error::Error>> {
        let queue = job.queue().unwrap_or_else(|| "default".to_string());
        let delay = job.delay().map(|secs| Duration::from_secs(secs));
        let wrapper = job::JobWrapper::new(&job)?;
        let payload = serde_json::to_string(&wrapper)?;
        Self::push(&queue, payload, delay).await
    }

    /// Create a new worker for processing jobs
    pub fn worker(queue: &str) -> worker::Worker {
        worker::Worker::new(queue)
    }
} 