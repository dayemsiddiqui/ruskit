use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cargo-kit")]
pub struct Cli {
    /// The name of the cargo subcommand (should be "kit")
    #[arg(hide = true)]
    pub kit: Option<String>,
    
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new Ruskit project
    New {
        /// Name of the project to create
        name: String,
    },
    /// Generate entities from database schema
    #[command(name = "entity:generate")]
    EntityGenerate,
    /// Start development server with hot reload
    Dev,
    /// Start production server
    Serve,
    /// Create a new model
    #[command(name = "make:model")]
    MakeModel {
        /// Name of the model to create
        name: String,
    },
    /// Create a new controller
    #[command(name = "make:controller")]
    MakeController {
        /// Name of the controller to create
        name: String,
    },
    /// Create a new DTO
    #[command(name = "make:dto")]
    MakeDto {
        /// Name of the DTO to create
        name: String,
    },
    /// Create model, DTO and controller
    #[command(name = "make:all")]
    MakeAll {
        /// Name to use for all components
        name: String,
    },
    /// Create a new Inertia page with controller and DTO
    #[command(name = "inertia:page")]
    InertiaPage {
        /// Name of the page to create (e.g., "Dashboard")
        name: String,
    },
    /// Create a new Inertia props type
    #[command(name = "inertia:prop")]
    InertiaProp {
        /// Name of the props to create (e.g., "Dashboard")
        name: String,
    },
} 