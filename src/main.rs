use std::net::SocketAddr;
use ruskit::web;
use tokio::net::TcpListener;
use std::fs;
use ruskit::framework::export_all_types;

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
    // Generate TypeScript types
    if let Err(e) = generate_typescript_types() {
        eprintln!("Error generating TypeScript types: {}", e);
    }

    // Get the router
    let app = web::routes().await;

    // Set up the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Server running on http://{}", addr);

    // Create the listener
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
