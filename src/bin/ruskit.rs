use clap::Parser;
use ruskit::framework::cli::{
    commands::Cli,
    handlers::handle_command,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    handle_command(&cli).await?;
    Ok(())
} 