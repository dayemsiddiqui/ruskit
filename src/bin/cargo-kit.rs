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
            Commands::New { name } => {
                if let Err(e) = ruskit::framework::cli::handlers::project::create_new_project(&name) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
            Commands::Serve => {
                if let Err(e) = ruskit::framework::cli::handlers::server::start_server().await {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
            Commands::MakeController { name } => {
                if let Err(e) = ruskit::framework::cli::handlers::make::make_controller(&name) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
            Commands::InertiaPage { name } => {
                println!("Creating Inertia page components for {}...", name);
                
                println!("\n1. Creating controller...");
                if let Err(e) = ruskit::framework::cli::handlers::make::make_page_controller(&name) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
                
                println!("\n2. Creating React component...");
                if let Err(e) = ruskit::framework::cli::handlers::make::make_page_component(&name) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
                
                println!("\nSuccessfully created Inertia page components!");
                println!("\nNext steps:");
                println!("1. Add your route in src/web.rs:");
                println!("   .route(\"/{}\", get({}Controller::show))", name.to_lowercase(), name);
                println!("2. Customize the page component in resources/js/pages/{}.tsx", name);
            }
            Commands::InertiaProp { name } => {
                println!("Creating Inertia props type for {}...", name);
                if let Err(e) = ruskit::framework::cli::handlers::make::make_page_dto(&name) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
                println!("\nSuccessfully created Inertia props type!");
            }
        },
    }
} 