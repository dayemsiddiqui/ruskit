use ruskit::app::jobs::TestJob;
use ruskit::framework::queue::Queue;
use ruskit::framework::bootstrap::app::bootstrap;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    // Initialize the application
    bootstrap().await
        .map_err(|e| format!("Failed to bootstrap application: {}", e))?;

    // Create and dispatch a test job
    let job = TestJob {
        message: "Hello from test job!".to_string(),
    };

    let job_id = Queue::dispatch(job).await?;
    println!("Dispatched test job with ID: {}", job_id);

    Ok(())
} 