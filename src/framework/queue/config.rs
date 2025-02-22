use sea_orm::DatabaseConnection;
use crate::framework::queue::{QueueDriver, QUEUE_DRIVER};
use crate::framework::queue::drivers::{DatabaseDriver, SqsDriver};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub enum QueueDriverType {
    Database,
    Sqs,
}

#[derive(Debug, Clone)]
pub struct QueueConfig {
    pub driver: QueueDriverType,
    pub sqs_region: Option<String>,
    pub sqs_queue_url: Option<String>,
    pub default_queue: String,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            driver: QueueDriverType::Database,
            sqs_region: None,
            sqs_queue_url: None,
            default_queue: "default".to_string(),
        }
    }
}

pub async fn init_queue(config: QueueConfig, db: DatabaseConnection) -> Result<(), String> {
    // If queue driver is already initialized, return early
    if QUEUE_DRIVER.get().is_some() {
        return Ok(());
    }

    let driver: Box<dyn QueueDriver + Send + Sync> = match config.driver {
        QueueDriverType::Database => Box::new(DatabaseDriver::new(db)),
        QueueDriverType::Sqs => {
            let region = config.sqs_region.ok_or("SQS region not configured")?;
            let queue_url = config.sqs_queue_url.ok_or("SQS queue URL not configured")?;
            Box::new(SqsDriver::new(&region, &queue_url).map_err(|e| e.to_string())?)
        }
    };

    QUEUE_DRIVER
        .set(Arc::new(RwLock::new(driver)))
        .map_err(|_| "Failed to initialize queue driver".to_string())
} 