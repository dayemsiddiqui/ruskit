use clap::{Parser, Subcommand};
use ruskit::framework::database::{
    migration::MigrationManager,
};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

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
    /// Create a new migration for an existing model
    #[command(name = "make:migration")]
    MakeMigration {
        /// Name of the migration (e.g., "add_email_to_users")
        name: String,
        /// Name of the model this migration is for
        #[arg(short, long)]
        model: String,
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
            if let Err(e) = run_server().await {
                eprintln!("Error starting server: {}", e);
                std::process::exit(1);
            }
        },
        Commands::MakeModel { name } => {
            println!("Creating model {}...", name);
            if let Err(e) = make_model(&name, true) { // Always create with migration
                eprintln!("Error creating model: {}", e);
                std::process::exit(1);
            }
        },
        Commands::MakeMigration { name, model } => {
            println!("Creating migration {} for model {}...", name, model);
            if let Err(e) = make_migration(&name, &model) {
                eprintln!("Error creating migration: {}", e);
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

    // Get current timestamp for migration ordering
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Generate model content with timestamped migration name
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
                "{timestamp}_create_{table_name}_table",
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

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    use ruskit::web;
    use std::net::SocketAddr;
    use tokio::net::TcpListener;

    // Get routes from web.rs
    let app = web::routes().await;

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    // Create the listener
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn make_migration(name: &str, model: &str) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let model_file = format!("src/app/models/{}.rs", model.to_lowercase());
    let path = Path::new(&model_file);
    
    if !path.exists() {
        return Err(format!("Model file {} does not exist", model_file).into());
    }
    
    // Read the current model file
    let mut content = fs::read_to_string(path)?;
    
    // Find the migrations vector
    if let Some(migrations_start) = content.find("fn migrations() -> Vec<Migration> {") {
        if let Some(vec_start) = content[migrations_start..].find("vec![") {
            let vec_end = content[migrations_start + vec_start..].find("]")
                .ok_or("Could not find end of migrations vector")?;
            
            // Insert the new migration at the end of the vector
            let insert_pos = migrations_start + vec_start + vec_end;
            let migration = format!(
                r#",
            Migration::new(
                "{timestamp}_{name}",
                "-- Add your UP migration SQL here",
                "-- Add your DOWN migration SQL here"
            )"#
            );
            
            content.insert_str(insert_pos, &migration);
            fs::write(path, content)?;
            
            println!("Created migration {timestamp}_{name}");
            println!("Please edit {} to add your migration SQL", model_file);
        }
    }
    
    Ok(())
} 