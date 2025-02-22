use crate::framework::queue::Job;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestJob {
    pub message: String,
}

#[async_trait]
impl Job for TestJob {
    fn queue(&self) -> Option<String> {
        Some("default".to_string())
    }

    fn delay(&self) -> Option<u64> {
        None
    }

    fn tries(&self) -> Option<u32> {
        Some(3)
    }

    async fn handle(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Processing TestJob with message: {}", self.message);
        Ok(())
    }

    fn serialize(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string(self)?)
    }

    fn deserialize(data: &str) -> Result<Box<dyn Job>, Box<dyn std::error::Error>> {
        let job: TestJob = serde_json::from_str(data)?;
        Ok(Box::new(job))
    }

    fn type_name() -> String {
        "TestJob".to_string()
    }
} 