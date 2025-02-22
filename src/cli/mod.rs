pub mod commands;
pub mod error;
pub mod handlers;

use commands::{Cli, Commands};
use error::CliError;
use handlers::{database, make, project, server};
use clap::Parser;

pub async fn run() -> Result<(), CliError> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::New { name } => {
            project::create_new_project(&name)?;
        },
        Commands::Migrate => {
            println!("Running migrations...");
            database::run_migrate().await?;
        },
        Commands::MigrateFresh => {
            println!("Dropping all tables and re-running migrations...");
            database::run_fresh().await?;
        },
        Commands::EntityGenerate => {
            println!("Generating entities from database schema...");
            database::generate_entities().await?;
        },
        Commands::Dev => {
            server::run_dev()?;
        },
        Commands::Serve => {
            println!("Starting production server...");
            server::run_server().await?;
        },
        Commands::MakeModel { name } => {
            println!("Creating model {}...", name);
            make::make_model(&name, true)?;
        },
        Commands::MakeMigration { name, model } => {
            println!("Creating migration {} for model {}...", name, model);
            make::make_migration(&name, &model)?;
        },
        Commands::MakeController { name } => {
            println!("Creating controller {}...", name);
            make::make_controller(&name)?;
        },
        Commands::MakeDto { name } => {
            println!("Creating DTO {}...", name);
            make::make_dto(&name)?;
        },
        Commands::MakeAll { name } => {
            println!("Creating all components for {}...", name);
            
            println!("\n1. Creating model...");
            make::make_model(&name, true)?;
            
            println!("\n2. Creating DTO...");
            make::make_dto(&name)?;
            
            println!("\n3. Creating controller...");
            make::make_controller(&name)?;
            
            println!("\nSuccessfully created all components!");
            println!("Run migrations with: cargo kit migrate");
        },
        Commands::InertiaPage { name } => {
            println!("Creating Inertia page components for {}...", name);
            
            println!("\n1. Creating DTO...");
            make::make_page_dto(&name)?;
            
            println!("\n2. Creating controller...");
            make::make_page_controller(&name)?;
            
            println!("\n3. Creating React component...");
            make::make_page_component(&name)?;
            
            println!("\nSuccessfully created Inertia page components!");
            println!("\nNext steps:");
            println!("1. Add your route in src/web.rs:");
            println!("   .route(\"/{}\", get({}Controller::show))", name.to_lowercase(), name);
            println!("2. Customize the page component in resources/js/pages/{}.tsx", name);
            println!("3. Add your data to the DTO in src/app/dtos/{}.rs", name.to_lowercase());
        },
        Commands::InertiaProp { name } => {
            println!("Creating Inertia props type for {}...", name);
            
            make::make_page_dto(&name)?;
            
            println!("\nSuccessfully created Inertia props type!");
            println!("\nNext steps:");
            println!("1. Add your props in src/app/dtos/{}.rs", name.to_lowercase());
            println!("2. Import the type in your component: import type {{ {}Props }} from '../types/generated';", name);
        },
    }
    
    Ok(())
} 