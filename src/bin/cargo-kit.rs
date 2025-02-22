use clap::{Parser, Subcommand};
use ruskit::framework::cli::commands::Commands;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    Kit(KitCli),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct KitCli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    let cli = CargoCli::parse();
    match cli {
        CargoCli::Kit(kit) => match kit.command {
            Commands::Dev => {
                if let Err(e) = ruskit::framework::cli::handlers::server::run_dev() {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
            Commands::Make { name, resource_type } => {
                if let Err(e) = ruskit::framework::cli::handlers::make::run_make(&name, resource_type) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        },
    }
} 