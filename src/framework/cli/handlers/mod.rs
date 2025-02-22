pub mod make;
pub mod project;
pub mod server;

pub use make::*;
pub use project::*;
pub use server::*;

use crate::framework::cli::commands::{Cli, Commands, ResourceType};
use crate::framework::cli::error::CliError;
use crate::framework::cli::handlers::make::run_make;
use crate::framework::cli::handlers::project::create_new_project;
use crate::framework::cli::handlers::server::start_server;

pub async fn handle_command(cli: &Cli) -> Result<(), CliError> {
    match &cli.command {
        Some(Commands::Schedule(cmd)) => {
            cmd.handle().await;
            Ok(())
        },
        Some(Commands::New { name }) => {
            create_new_project(name)?;
            Ok(())
        },
        Some(Commands::Dev) => {
            start_server(true).await?;
            Ok(())
        },
        Some(Commands::Serve) => {
            start_server(false).await?;
            Ok(())
        },
        Some(Commands::MakeController { name }) => {
            run_make(name, ResourceType::Controller)?;
            Ok(())
        },
        Some(Commands::InertiaPage { name }) => {
            make_page_dto(name)?;
            make_page_controller(name)?;
            make_page_component(name)?;
            Ok(())
        },
        Some(Commands::InertiaProp { name }) => {
            make_page_dto(name)?;
            Ok(())
        },
        Some(Commands::Make { name, resource_type }) => {
            run_make(name, resource_type.clone())?;
            Ok(())
        },
        None => Ok(()),
    }
} 