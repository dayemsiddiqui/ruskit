pub mod commands;
pub mod error;
pub mod handlers;

use commands::{Cli, Commands};
use error::CliError;
use handlers::{make, project, server};
use clap::Parser;

pub async fn run() -> Result<(), CliError> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::New { name } => {
            project::create_new_project(&name)?;
        },
        Commands::Dev => {
            server::run_dev()?;
        },
        Commands::Serve => {
            println!("Starting production server...");
            server::run_server().await?;
        },
        Commands::MakeController { name } => {
            println!("Creating controller {}...", name);
            make::make_controller(&name)?;
        },
        Commands::InertiaPage { name } => {
            println!("Creating Inertia page components for {}...", name);
            
            println!("\n1. Creating controller...");
            make::make_page_controller(&name)?;
            
            println!("\n2. Creating React component...");
            make::make_page_component(&name)?;
            
            println!("\nSuccessfully created Inertia page components!");
            println!("\nNext steps:");
            println!("1. Add your route in src/web.rs:");
            println!("   .route(\"/{}\", get({}Controller::show))", name.to_lowercase(), name);
            println!("2. Customize the page component in resources/js/pages/{}.tsx", name);
        },
        Commands::InertiaProp { name } => {
            println!("Creating Inertia props type for {}...", name);
            make::make_page_dto(&name)?;
            println!("\nSuccessfully created Inertia props type!");
        },
    }
    
    Ok(())
} 