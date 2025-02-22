use std::net::SocketAddr;
use ruskit::{web, setup};
use tokio::net::TcpListener;
use std::fs;
use ruskit::framework::export_all_types;
use dotenvy::dotenv;
use axum_inertia::{InertiaConfig, vite};
use ruskit::web::AppState;
use sea_orm::DatabaseConnection;

fn generate_typescript_types() -> std::io::Result<()> {
    // Ensure the types directory exists
    fs::create_dir_all("resources/js/types")?;
    
    // Create or truncate the generated.ts file
    let output_file = "resources/js/types/generated.ts";
    fs::write(output_file, "")?;

    // Generate types for all DTOs
    if let Err(e) = export_all_types(output_file) {
        eprintln!("Error generating TypeScript types: {}", e);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // Load .env file
    if let Err(e) = dotenv() {
        eprintln!("Warning: Error loading .env file: {}", e);
    }

    // Setup application
    if let Err(e) = setup().await {
        eprintln!("Error setting up application: {}", e);
        std::process::exit(1);
    }

    // Generate TypeScript types
    if let Err(e) = generate_typescript_types() {
        eprintln!("Error generating TypeScript types: {}", e);
    }

    // Initialize the application and get the database connection
    let db = ruskit::bootstrap::app::bootstrap().await.unwrap();
    
    // Create the app state
    let app_state = AppState {
        db,
        inertia: vite::Development::default()
            .port(3000)
            .main("resources/js/app.jsx")
            .lang("en")
            .title("Ruskit")
            .into_config(),
    };

    // Get the router and add state
    let app = web::routes().await.with_state(app_state);

    // Set up the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Server running on http://{}", addr);

    // Create the listener
    let listener = TcpListener::bind(addr).await.unwrap();
    let service = app.into_make_service();
    
    axum::serve(listener, service).await.unwrap();
}
