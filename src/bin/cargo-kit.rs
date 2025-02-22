use clap::Parser;
use ruskit::framework::cli::commands::{Cli, Commands};
use ruskit::framework::cli::handlers::handle_command;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    Kit(Cli),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct KitCli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let CargoCli::Kit(cli) = CargoCli::parse();
    handle_command(&cli).await?;
    Ok(())
} 