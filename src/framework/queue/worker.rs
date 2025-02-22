use std::time::Duration;
use tokio::time::sleep;
use crate::framework::queue::{Queue, job::JobWrapper};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

type JobFactory = Box<dyn Fn(&str) -> Result<Box<dyn crate::framework::queue::Job>, Box<dyn std::error::Error>> + Send + Sync>;

static JOB_FACTORIES: Lazy<Arc<RwLock<HashMap<String, JobFactory>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(HashMap::new()))
});

pub struct Worker {
    queue: String,
    sleep_duration: Duration,
    max_tries: u32,
    pub running: Arc<AtomicBool>,
}

impl Worker {
    pub fn new(queue: &str) -> Self {
        Self {
            queue: queue.to_string(),
            sleep_duration: Duration::from_secs(1),
            max_tries: 3,
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn sleep_duration(mut self, duration: Duration) -> Self {
        self.sleep_duration = duration;
        self
    }

    pub fn max_tries(mut self, tries: u32) -> Self {
        self.max_tries = tries;
        self
    }

    pub async fn run(&self) {
        println!("Starting queue worker for queue: {}", self.queue);
        
        while self.running.load(Ordering::SeqCst) {
            match Queue::pop(&self.queue).await {
                Some(queued_job) => {
                    println!("Processing job {} from queue {}", queued_job.id, queued_job.queue);
                    
                    // Check if we should process this job based on attempts
                    if queued_job.attempts > self.max_tries {
                        println!("Job {} has exceeded maximum attempts ({}), deleting", queued_job.id, self.max_tries);
                        if let Err(e) = Queue::delete(queued_job.id).await {
                            eprintln!("Failed to delete failed job {}: {}", queued_job.id, e);
                        }
                        continue;
                    }

                    // Process the job
                    match serde_json::from_str::<JobWrapper>(&queued_job.payload) {
                        Ok(wrapper) => {
                            match process_job(&wrapper).await {
                                Ok(_) => {
                                    println!("Job {} completed successfully", queued_job.id);
                                    if let Err(e) = Queue::delete(queued_job.id).await {
                                        eprintln!("Failed to delete completed job {}: {}", queued_job.id, e);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Job {} failed: {}", queued_job.id, e);
                                    // Release the job back to the queue with a delay
                                    if let Err(e) = Queue::release(queued_job.id, Some(Duration::from_secs(30))).await {
                                        eprintln!("Failed to release failed job {}: {}", queued_job.id, e);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to deserialize job payload: {}", e);
                            if let Err(e) = Queue::delete(queued_job.id).await {
                                eprintln!("Failed to delete malformed job {}: {}", queued_job.id, e);
                            }
                        }
                    }
                }
                None => {
                    // No jobs available, sleep before checking again
                    sleep(self.sleep_duration).await;
                }
            }
        }
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
}

/// Register a job type for processing
pub fn register_job<T: crate::framework::queue::Job + 'static>() {
    let job_type = T::type_name();
    let factory: JobFactory = Box::new(move |data| {
        T::deserialize(data)
    });

    tokio::spawn(async move {
        let mut factories = JOB_FACTORIES.write().await;
        factories.insert(job_type, factory);
    });
}

async fn process_job(wrapper: &JobWrapper) -> Result<(), Box<dyn std::error::Error>> {
    let factories = JOB_FACTORIES.read().await;
    
    if let Some(factory) = factories.get(&wrapper.job_type) {
        let job = factory(&wrapper.job_data)?;
        job.handle().await?;
        Ok(())
    } else {
        Err(format!("No job factory registered for type: {}", wrapper.job_type).into())
    }
} 