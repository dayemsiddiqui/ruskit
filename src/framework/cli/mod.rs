pub mod commands;
pub mod error;
pub mod handlers;

use crate::framework::cli::commands::{Cli, Commands};
use crate::framework::cli::error::CliError;

pub async fn run_cli(cli: Cli) -> Result<(), CliError> {
    match cli.command {
        Some(Commands::Schedule(cmd)) => {
            cmd.handle().await;
            Ok(())
        },
        Some(Commands::New { name }) => {
            handlers::project::create_new_project(&name)?;
            Ok(())
        },
        Some(Commands::Dev) => {
            handlers::server::start_server(true).await?;
            Ok(())
        },
        Some(Commands::Serve) => {
            handlers::server::start_server(false).await?;
            Ok(())
        },
        Some(Commands::MakeController { name }) => {
            handlers::make::run_make(&name, commands::ResourceType::Controller)?;
            Ok(())
        },
        Some(Commands::InertiaPage { name }) => {
            handlers::make::make_page_dto(&name)?;
            handlers::make::make_page_controller(&name)?;
            handlers::make::make_page_component(&name)?;
            Ok(())
        },
        Some(Commands::InertiaProp { name }) => {
            handlers::make::make_page_dto(&name)?;
            Ok(())
        },
        Some(Commands::Make { name, resource_type }) => {
            handlers::make::run_make(&name, resource_type)?;
            Ok(())
        },
        None => Ok(()),
    }
} 