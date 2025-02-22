use clap::{Parser, Subcommand};
use std::error::Error;
use std::time::Duration;
use ruskit::framework::run;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the development server
    Serve,
    /// Run the scheduler
    Schedule,
    /// Run the queue worker
    #[command(name = "queue:work")]
    QueueWork {
        /// The queue to process
        #[arg(short, long, default_value = "default")]
        queue: String,
        /// Sleep duration in seconds between polling for new jobs
        #[arg(short, long, default_value = "1")]
        sleep: u64,
        /// Maximum number of times to attempt a job
        #[arg(short, long, default_value = "3")]
        tries: u32,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve => {
            run().await?;
        }
        Commands::Schedule => {
            ruskit::app::console::kernel::schedule().await;
        }
        Commands::QueueWork { queue, sleep, tries } => {
            // Initialize the application
            ruskit::framework::bootstrap::app::bootstrap().await
                .map_err(|e| format!("Failed to bootstrap application: {}", e))?;

            // Create and run the worker
            let worker = ruskit::framework::queue::Queue::worker(&queue)
                .sleep_duration(Duration::from_secs(sleep))
                .max_tries(tries);

            // Handle Ctrl+C gracefully
            let worker_running = worker.running.clone();
            ctrlc::set_handler(move || {
                println!("\nReceived Ctrl+C, shutting down worker...");
                worker_running.store(false, std::sync::atomic::Ordering::SeqCst);
            })?;

            worker.run().await;
        }
    }

    Ok(())
} 