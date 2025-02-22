use clap::{Parser, Subcommand};
use std::fmt;

#[derive(Parser)]
#[command(name = "cargo-kit")]
pub struct Cli {
    /// The name of the cargo subcommand (should be "kit")
    #[arg(hide = true)]
    pub kit: Option<String>,
    
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a new Ruskit project
    New {
        /// Name of the project to create
        name: String,
    },
    /// Start development server with hot reload
    Dev,
    /// Start production server
    Serve,
    /// Create a new controller
    #[command(name = "make:controller")]
    MakeController {
        /// Name of the controller to create
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
    /// Make a new resource
    Make {
        /// Name of the resource
        name: String,
        /// Type of resource to create
        #[arg(value_enum)]
        resource_type: ResourceType,
    },
}

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum ResourceType {
    Controller,
    Model,
    Migration,
    Middleware,
    Command,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceType::Controller => write!(f, "controller"),
            ResourceType::Model => write!(f, "model"),
            ResourceType::Migration => write!(f, "migration"),
            ResourceType::Middleware => write!(f, "middleware"),
            ResourceType::Command => write!(f, "command"),
        }
    }
} 