use clap::{Parser, Subcommand};
use std::fmt;
pub mod schedule;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project
    New {
        name: String,
    },
    /// Run the development server
    Dev,
    /// Run the production server
    Serve,
    /// Create a new controller
    MakeController {
        name: String,
    },
    /// Create a new Inertia page
    InertiaPage {
        name: String,
    },
    /// Create a new Inertia prop
    InertiaProp {
        name: String,
    },
    /// Make a new resource
    Make {
        name: String,
        resource_type: ResourceType,
    },
    /// Run the scheduler
    Schedule(schedule::ScheduleCommand),
}

#[derive(clap::ValueEnum, Clone)]
pub enum ResourceType {
    Controller,
    Model,
    Migration,
    Seeder,
    Factory,
    Test,
    Command,
    Event,
    Job,
    Mail,
    Notification,
    Policy,
    Provider,
    Request,
    Resource,
    Rule,
    Scope,
    Service,
    Middleware,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceType::Controller => write!(f, "controller"),
            ResourceType::Model => write!(f, "model"),
            ResourceType::Migration => write!(f, "migration"),
            ResourceType::Seeder => write!(f, "seeder"),
            ResourceType::Factory => write!(f, "factory"),
            ResourceType::Test => write!(f, "test"),
            ResourceType::Command => write!(f, "command"),
            ResourceType::Event => write!(f, "event"),
            ResourceType::Job => write!(f, "job"),
            ResourceType::Mail => write!(f, "mail"),
            ResourceType::Notification => write!(f, "notification"),
            ResourceType::Policy => write!(f, "policy"),
            ResourceType::Provider => write!(f, "provider"),
            ResourceType::Request => write!(f, "request"),
            ResourceType::Resource => write!(f, "resource"),
            ResourceType::Rule => write!(f, "rule"),
            ResourceType::Scope => write!(f, "scope"),
            ResourceType::Service => write!(f, "service"),
            ResourceType::Middleware => write!(f, "middleware"),
        }
    }
} 