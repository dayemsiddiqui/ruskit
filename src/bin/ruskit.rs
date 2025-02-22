use ruskit::framework;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    framework::run().await
} 