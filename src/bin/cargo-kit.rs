#[tokio::main]
async fn main() {
    if let Err(e) = ruskit::cli::run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
} 