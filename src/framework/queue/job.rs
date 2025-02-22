use async_trait::async_trait;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use std::any::Any;

/// A trait that represents a job that can be queued and processed
#[async_trait]
pub trait Job: Send + Sync {
    /// The name of the queue this job should be dispatched to
    fn queue(&self) -> Option<String> {
        None
    }

    /// The number of seconds to wait before making the job available
    fn delay(&self) -> Option<u64> {
        None
    }

    /// The number of times the job may be attempted
    fn tries(&self) -> Option<u32> {
        None
    }

    /// Handle the job
    async fn handle(&self) -> Result<(), Box<dyn std::error::Error>>;

    /// Serialize the job to a string
    fn serialize(&self) -> Result<String, Box<dyn std::error::Error>>;

    /// Create a job from a serialized string
    fn deserialize(data: &str) -> Result<Box<dyn Job>, Box<dyn std::error::Error>> where Self: Sized;

    /// Get the type name of the job
    fn type_name() -> String where Self: Sized;
}

/// A wrapper around a job that includes metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct JobWrapper {
    /// The type name of the job
    pub job_type: String,
    /// The serialized job data
    pub job_data: String,
}

impl JobWrapper {
    pub fn new<T: Job>(job: &T) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            job_type: T::type_name(),
            job_data: job.serialize()?,
        })
    }
} 