use clap::{Parser, Subcommand};
use ruskit::framework::database::{
    migration::MigrationManager,
};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "cargo-kit")]
struct Cli {
    /// The name of the cargo subcommand (should be "kit")
    #[arg(hide = true)]
    kit: Option<String>,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run database migrations
    Migrate,
    /// Start development server with hot reload
    Dev,
    /// Start production server
    Serve,
    /// Create a new model with migration
    #[command(name = "make:model")]
    MakeModel {
        /// Name of the model to create
        name: String,
    },
}

#[derive(Subcommand)]
enum MakeType {
    /// Create a new model
    Model {
        /// Name of the model to create
        name: String,
        /// Create a migration for the model
        #[arg(short, long)]
        migration: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Migrate => {
            println!("Running migrations...");
            if let Err(e) = run_migrate().await {
                eprintln!("Error running migrations: {}", e);
                std::process::exit(1);
            }
        },
        Commands::Dev => {
            println!("Starting development server...");
            // TODO: Implement dev server with hot reload
        },
        Commands::Serve => {
            println!("Starting production server...");
            // TODO: Implement production server
        },
        Commands::MakeModel { name } => {
            println!("Creating model {}...", name);
            if let Err(e) = make_model(&name, true) { // Always create with migration
                eprintln!("Error creating model: {}", e);
                std::process::exit(1);
            }
        },
    }
}

fn make_model(name: &str, with_migration: bool) -> Result<(), Box<dyn std::error::Error>> {
    let models_dir = Path::new("src/app/models");
    if !models_dir.exists() {
        fs::create_dir_all(models_dir)?;
    }

    let model_name = name.to_string();
    let table_name = inflector::string::pluralize::to_plural(&model_name.to_lowercase());
    let model_file = models_dir.join(format!("{}.rs", model_name.to_lowercase()));

    // Generate model content
    let model_content = format!(
        r#"use serde::{{Deserialize, Serialize}};
use sqlx::FromRow;
use crate::framework::database::{{
    model::Model,
    query_builder::QueryBuilder,
    DatabaseError,
    migration::Migration,
}};
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct {model_name} {{
    pub id: i64,
    // TODO: Add your fields here
    pub created_at: i64,
    pub updated_at: i64,
}}

#[async_trait]
impl Model for {model_name} {{
    fn table_name() -> &'static str {{
        "{table_name}"
    }}

    fn migrations() -> Vec<Migration> {{
        vec![
            Migration::new(
                "create_{table_name}_table",
                "CREATE TABLE {table_name} (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    -- TODO: Add your columns here
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                )",
                "DROP TABLE {table_name}"
            ),
        ]
    }}
}}

impl {model_name} {{
    // TODO: Add your custom query methods here
    
    /// Get recent records
    pub async fn recent(limit: i64) -> Result<Vec<Self>, DatabaseError> {{
        QueryBuilder::table(Self::table_name())
            .order_by("created_at", "DESC")
            .limit(limit)
            .get::<Self>()
            .await
    }}
}}"#
    );

    // Write model file
    fs::write(&model_file, model_content)?;
    println!("Created model file: {}", model_file.display());

    // Update mod.rs to include the new model
    let mod_file = models_dir.join("mod.rs");
    let mut mod_content = String::new();
    
    if mod_file.exists() {
        mod_content = fs::read_to_string(&mod_file)?;
    }

    // Add mod declaration if not exists
    let mod_line = format!("mod {};", model_name.to_lowercase());
    if !mod_content.contains(&mod_line) {
        if !mod_content.is_empty() {
            mod_content.push('\n');
        }
        mod_content.push_str(&mod_line);
    }

    // Add pub use if not exists
    let use_line = format!("pub use {}::{};", model_name.to_lowercase(), model_name);
    if !mod_content.contains(&use_line) {
        if !mod_content.is_empty() {
            mod_content.push('\n');
        }
        mod_content.push_str(&use_line);
    }

    fs::write(mod_file, mod_content)?;

    if with_migration {
        println!("Don't forget to run migrations with: cargo kit migrate");
    }

    Ok(())
}

async fn run_migrate() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MigrationManager::new().await?;
    manager.run(manager.get_all_model_migrations()).await?;
    println!("Migrations completed successfully.");
    Ok(())
}

async fn run_rollback() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MigrationManager::new().await?;
    manager.rollback().await?;
    println!("Rollback completed successfully.");
    Ok(())
}

async fn run_fresh() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MigrationManager::new().await?;
    manager.refresh().await?;
    println!("Database refresh completed successfully.");
    Ok(())
} 