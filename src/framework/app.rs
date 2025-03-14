use std::net::SocketAddr;
use std::fs;
use tokio::net::TcpListener;
use axum::serve;
use crate::framework::typescript::export_all_types;
use dotenvy::dotenv;

/// Generate TypeScript types for all DTOs
pub fn generate_typescript_types() -> std::io::Result<()> {
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

/// Run the application
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(&addr).await?;
    println!("Server running on {}", addr);

    // Load .env file
    if let Err(e) = dotenv() {
        eprintln!("Warning: Error loading .env file: {}", e);
    }

    // Generate TypeScript types
    if let Err(e) = generate_typescript_types() {
        eprintln!("Error generating TypeScript types: {}", e);
    }

    // Initialize the application and get the database connection
    let db = match crate::framework::bootstrap::app::bootstrap().await {
        Ok(db) => {
            println!("Application bootstrapped successfully");
            db
        },
        Err(e) => {
            eprintln!("Failed to bootstrap application: {}", e);
            std::process::exit(1);
        }
    };
    
    // Get the router and add state
    let app = crate::web::routes(db).await;
    
    println!("Starting server...");
    serve(listener, app).await?;
    
    Ok(())
} 