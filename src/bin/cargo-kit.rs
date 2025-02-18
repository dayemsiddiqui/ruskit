use clap::{Parser, Subcommand};
use ruskit::framework::database::{
    migration::MigrationManager,
    initialize,
};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use console::style;

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
    /// Drop all tables and re-run all migrations
    #[command(name = "migrate:fresh")]
    MigrateFresh,
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
    /// Create a new factory for a model
    #[command(name = "make:factory")]
    MakeFactory {
        /// Name of the model to create factory for
        name: String,
    },
    /// Create a new seeder
    #[command(name = "make:seeder")]
    MakeSeeder {
        /// Name of the seeder to create
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
    /// Seed the database with sample data
    #[command(name = "db:seed")]
    DbSeed,
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
        Commands::MigrateFresh => {
            println!("Dropping all tables and re-running migrations...");
            if let Err(e) = run_fresh().await {
                eprintln!("Error refreshing database: {}", e);
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
        Commands::MakeFactory { name } => {
            println!("Creating factory for {}...", name);
            if let Err(e) = make_factory(&name) {
                eprintln!("Error creating factory: {}", e);
                std::process::exit(1);
            }
        },
        Commands::MakeSeeder { name } => {
            println!("Creating seeder {}...", name);
            if let Err(e) = make_seeder(&name) {
                eprintln!("Error creating seeder: {}", e);
                std::process::exit(1);
            }
        },
        Commands::MakeController { name } => {
            println!("Creating controller {}...", name);
            if let Err(e) = make_controller(&name) {
                eprintln!("Error creating controller: {}", e);
                std::process::exit(1);
            }
        },
        Commands::MakeDto { name } => {
            println!("Creating DTO {}...", name);
            if let Err(e) = make_dto(&name) {
                eprintln!("Error creating DTO: {}", e);
                std::process::exit(1);
            }
        },
        Commands::DbSeed => {
            use ruskit::framework::database::seeder::DatabaseSeeder;
            use ruskit::app;
            
            println!("Initializing application...");
            app::initialize();
            
            println!("Initializing database...");
            let db_config = ruskit::framework::database::config::DatabaseConfig::from_env();
            if let Err(e) = initialize(Some(db_config)).await {
                eprintln!("Error initializing database: {}", e);
                std::process::exit(1);
            }

            println!("Running migrations...");
            let manager = MigrationManager::new().await.unwrap();
            if let Err(e) = manager.run(manager.get_all_model_migrations()).await {
                eprintln!("Error running migrations: {}", e);
                std::process::exit(1);
            }
            
            println!("Seeding database...");
            if let Err(e) = DatabaseSeeder::run_all().await {
                eprintln!("Error seeding database: {}", e);
                std::process::exit(1);
            }
            println!("Database seeded successfully!");
        }
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
    fn id(&self) -> i64 {{
        self.id
    }}

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
    use ruskit::app;
    
    println!("Initializing application...");
    app::initialize();
    
    // Initialize database connection
    println!("Initializing database...");
    let db_config = ruskit::framework::database::config::DatabaseConfig::from_env();
    initialize(Some(db_config)).await?;
    
    // Run migrations
    println!("Running migrations...");
    let manager = MigrationManager::new().await?;
    manager.run(manager.get_all_model_migrations()).await?;
    println!("Migrations completed successfully");
    Ok(())
}

async fn run_rollback() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MigrationManager::new().await?;
    manager.rollback().await?;
    println!("Rollback completed successfully.");
    Ok(())
}

async fn run_fresh() -> Result<(), Box<dyn std::error::Error>> {
    use ruskit::app;
    
    println!("Initializing application...");
    app::initialize();
    
    // Initialize database connection
    println!("Initializing database...");
    let db_config = ruskit::framework::database::config::DatabaseConfig::from_env();
    initialize(Some(db_config)).await?;
    
    // Drop all tables
    let manager = MigrationManager::new().await?;
    println!("Dropping all tables...");
    manager.drop_all_tables().await?;
    println!("All tables dropped successfully");
    
    // Run migrations
    println!("Running fresh migrations...");
    manager.run(manager.get_all_model_migrations()).await?;
    println!("Fresh migrations completed successfully");
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
            // Find the actual end of the vector by counting brackets
            let vec_content = &content[migrations_start + vec_start..];
            let mut bracket_count = 0;
            let mut vec_end = 0;
            
            for (i, c) in vec_content.chars().enumerate() {
                match c {
                    '[' => bracket_count += 1,
                    ']' => {
                        bracket_count -= 1;
                        if bracket_count == 0 {
                            vec_end = i;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            
            if vec_end == 0 {
                return Err("Could not find proper end of migrations vector".into());
            }
            
            // Find the last Migration::new in the vector
            let last_migration_pos = vec_content[..vec_end]
                .rfind("Migration::new")
                .unwrap_or(vec_end);
            
            // If we found a previous migration, insert after its closing parenthesis
            let insert_pos = if last_migration_pos < vec_end {
                let remaining = &vec_content[last_migration_pos..vec_end];
                if let Some(paren_end) = remaining.find("),") {
                    migrations_start + vec_start + last_migration_pos + paren_end + 1
                } else {
                    migrations_start + vec_start + vec_end
                }
            } else {
                migrations_start + vec_start + vec_end
            };
            
            // Insert the new migration
            let migration = if content[..insert_pos].trim_end().ends_with(',') {
                format!(
                    r#"
            Migration::new(
                "{timestamp}_{name}",
                "-- Add your UP migration SQL here",
                "-- Add your DOWN migration SQL here"
            )"#
                )
            } else {
                format!(
                    r#",
            Migration::new(
                "{timestamp}_{name}",
                "-- Add your UP migration SQL here",
                "-- Add your DOWN migration SQL here"
            )"#
                )
            };
            
            content.insert_str(insert_pos, &migration);
            fs::write(path, content)?;
            
            println!("Created migration {timestamp}_{name}");
            println!("Please edit {} to add your migration SQL", model_file);
        }
    }
    
    Ok(())
}

fn make_factory(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let factories_dir = Path::new("src/app/factories");
    if !factories_dir.exists() {
        fs::create_dir_all(factories_dir)?;
    }

    let model_name = name.to_string();
    let factory_file = factories_dir.join(format!("{}_factory.rs", model_name.to_lowercase()));

    let factory_content = format!(
        r#"use crate::app::models::{model_name};
use crate::framework::database::factory::Factory;
use crate::framework::database::DatabaseError;
use fake::{{faker::internet::en::*, faker::name::en::*, Fake}};
use serde_json::json;
use std::time::{{SystemTime, UNIX_EPOCH}};

impl Factory for {model_name} {{
    fn definition() -> serde_json::Value {{
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        json!({{
            // TODO: Add your fake data here
            "created_at": now,
            "updated_at": now
        }})
    }}
}}"#
    );

    fs::write(&factory_file, factory_content)?;
    println!("Created factory file: {}", factory_file.display());

    // Update mod.rs
    let mod_file = factories_dir.join("mod.rs");
    let mut mod_content = String::new();
    
    if mod_file.exists() {
        mod_content = fs::read_to_string(&mod_file)?;
    }

    let mod_line = format!("pub mod {}_factory;", model_name.to_lowercase());
    if !mod_content.contains(&mod_line) {
        if !mod_content.is_empty() {
            mod_content.push('\n');
        }
        mod_content.push_str(&mod_line);
    }

    fs::write(mod_file, mod_content)?;
    Ok(())
}

fn make_seeder(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let seeders_dir = Path::new("src/app/seeders");
    if !seeders_dir.exists() {
        fs::create_dir_all(seeders_dir)?;
    }

    let seeder_name = if name.ends_with("Seeder") {
        name.to_string()
    } else {
        format!("{}Seeder", name)
    };
    
    let seeder_file = seeders_dir.join(format!("{}.rs", seeder_name.to_lowercase()));

    let seeder_content = format!(
        r#"use crate::framework::database::seeder::{{Seeder, DatabaseSeeder}};
use crate::framework::database::DatabaseError;
use once_cell::sync::Lazy;

pub struct {seeder_name};

#[async_trait::async_trait]
impl Seeder for {seeder_name} {{
    async fn run(&self) -> Result<(), DatabaseError> {{
        // TODO: Add your seeding logic here
        Ok(())
    }}
}}

static SEEDER: Lazy<()> = Lazy::new(|| {{
    DatabaseSeeder::register(Box::new({seeder_name}));
}});"#
    );

    fs::write(&seeder_file, seeder_content)?;
    println!("Created seeder file: {}", seeder_file.display());

    // Update mod.rs
    let mod_file = seeders_dir.join("mod.rs");
    let mut mod_content = String::new();
    
    if mod_file.exists() {
        mod_content = fs::read_to_string(&mod_file)?;
    }

    let mod_line = format!("pub mod {};", seeder_name.to_lowercase());
    if !mod_content.contains(&mod_line) {
        if !mod_content.is_empty() {
            mod_content.push('\n');
        }
        mod_content.push_str(&mod_line);
    }

    fs::write(mod_file, mod_content)?;
    Ok(())
}

fn make_controller(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let controller_dir = Path::new("src/app/controllers");
    if !controller_dir.exists() {
        fs::create_dir_all(controller_dir)?;
    }
    
    let controller_name = if name.ends_with("Controller") {
        name.to_string()
    } else {
        format!("{}Controller", name)
    };
    
    // Check if model and DTO files exist
    let model_exists = Path::new("src/app/models")
        .join(format!("{}.rs", name.to_lowercase()))
        .exists();
    let dto_exists = Path::new("src/app/dtos")
        .join(format!("{}.rs", name.to_lowercase()))
        .exists();
    
    // Convert to snake case: UserController -> user_controller
    let file_name = controller_dir.join(format!("{}_controller.rs", name.to_lowercase()));
    
    // Build imports based on what exists
    let mut imports = String::from(r#"use axum::{
    response::Json,
    extract::Path,
};"#);
    
    if model_exists {
        imports.push_str(&format!("\nuse crate::app::models::{};", name));
        imports.push_str("\nuse crate::framework::database::model::Model;");
    }
    
    if dto_exists {
        imports.push_str(&format!("\nuse crate::app::dtos::{}::{{Create{}Request, {}Response, {}ListResponse}};", 
            name.to_lowercase(), name, name, name));
    }

    let controller_content = format!(r#"{imports}

/// {} Controller handling all {}-related endpoints
pub struct {} {{}}

impl {} {{"#, 
        name, name.to_lowercase(),
        controller_name, controller_name
    );

    // Add methods based on what's available
    let mut methods = String::new();
    
    if model_exists && dto_exists {
        methods.push_str(&format!(r#"
    /// Returns a list of {}s
    pub async fn index() -> Json<{}ListResponse> {{
        match {}::all().await {{
            Ok(items) => Json({}ListResponse::from(items)),
            Err(e) => panic!("Database error: {{}}", e), // In a real app, use proper error handling
        }}
    }}

    /// Returns details for a specific {}
    pub async fn show(Path(id): Path<i64>) -> Json<Option<{}Response>> {{
        match {}::find(id).await {{
            Ok(Some(item)) => Json(Some({}Response::from(item))),
            Ok(None) => Json(None),
            Err(e) => panic!("Database error: {{}}", e), // In a real app, use proper error handling
        }}
    }}

    /// Creates a new {}
    pub async fn store(Json(payload): Json<Create{}Request>) -> Json<{}Response> {{
        let item: {} = payload.into();
        match {}::create(item).await {{
            Ok(created) => Json({}Response::from(created)),
            Err(e) => panic!("Database error: {{}}", e), // In a real app, use proper error handling
        }}
    }}"#,
            name.to_lowercase(), name, name, name,
            name.to_lowercase(), name, name, name,
            name.to_lowercase(), name, name, name, name, name
        ));
    } else {
        methods.push_str("\n    // TODO: Add your controller methods here\n");
        if !model_exists {
            methods.push_str(&format!("    // Note: Create a model first using: cargo kit make:model {}\n", name));
        }
        if !dto_exists {
            methods.push_str(&format!("    // Note: Create DTOs first using: cargo kit make:dto {}\n", name));
        }
    }

    let controller_content = format!("{}\n{}\n}}", controller_content, methods);

    fs::write(&file_name, controller_content)?;

    // Update mod.rs with snake case module name
    let mod_file = controller_dir.join("mod.rs");
    let mut mod_content = String::new();
    
    if mod_file.exists() {
        mod_content = fs::read_to_string(&mod_file)?;
    }

    let mod_name = format!("{}_controller", name.to_lowercase());
    let mod_line = format!("pub mod {};\npub use {}::*;\n", mod_name, mod_name);
    if !mod_content.contains(&mod_line) {
        mod_content.push_str(&mod_line);
    }

    fs::write(mod_file, mod_content)?;
    println!("Created controller file: {}", file_name.display());
    
    // Print helpful messages about missing dependencies
    if !model_exists {
        println!("{}", style(format!("Note: Model '{}' does not exist. Create it with: cargo kit make:model {}", name, name)).yellow());
    }
    if !dto_exists {
        println!("{}", style(format!("Note: DTO '{}' does not exist. Create it with: cargo kit make:dto {}", name, name)).yellow());
    }
    
    Ok(())
}

fn make_dto(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dto_dir = Path::new("src/app/dtos");
    if !dto_dir.exists() {
        fs::create_dir_all(dto_dir)?;
    }
    
    // Use base name: User -> user.rs
    let file_name = dto_dir.join(format!("{}.rs", name.to_lowercase()));
    
    // Check if model exists
    let model_exists = Path::new("src/app/models")
        .join(format!("{}.rs", name.to_lowercase()))
        .exists();
    
    let mut imports = String::from("use serde::{Serialize, Deserialize};\nuse validator::Validate;");
    
    if model_exists {
        imports.push_str(&format!("\nuse crate::app::models::{};", name));
    }
    
    let dto_content = format!(r#"{imports}

#[derive(Serialize)]
pub struct {name}Response {{
    pub id: i64,
    // Add your fields here
    pub created_at: i64,
    pub updated_at: i64,
}}

#[derive(Deserialize, Validate)]
pub struct Create{name}Request {{
    // Add your validation fields here
}}

#[derive(Serialize)]
pub struct {name}ListResponse {{
    pub data: Vec<{name}Response>,
}}

impl From<Vec<{name}>> for {name}ListResponse {{
    fn from(items: Vec<{name}>) -> Self {{
        Self {{
            data: items.into_iter().map({name}Response::from).collect(),
        }}
    }}
}}

impl From<{name}> for {name}Response {{
    fn from(item: {name}) -> Self {{
        Self {{
            id: item.id,
            // Map your fields here
            created_at: item.created_at,
            updated_at: item.updated_at,
        }}
    }}
}}

impl From<Create{name}Request> for {name} {{
    fn from(req: Create{name}Request) -> Self {{
        use std::time::{{SystemTime, UNIX_EPOCH}};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
            
        Self {{
            id: 0,
            // Map your fields here
            created_at: now,
            updated_at: now,
        }}
    }}
}}"#, name=name);

    fs::write(&file_name, dto_content)?;

    // Update mod.rs with just the base name
    let mod_file = dto_dir.join("mod.rs");
    let mut mod_content = String::new();
    
    if mod_file.exists() {
        mod_content = fs::read_to_string(&mod_file)?;
    }

    let mod_line = format!("pub mod {};\n", name.to_lowercase());
    if !mod_content.contains(&mod_line) {
        mod_content.push_str(&mod_line);
    }

    fs::write(mod_file, mod_content)?;
    println!("Created DTO file: {}", file_name.display());
    
    // Print helpful message if model doesn't exist
    if !model_exists {
        println!("{}", style(format!("Note: Model '{}' does not exist. Create it with: cargo kit make:model {}", name, name)).yellow());
    }
    
    Ok(())
} 