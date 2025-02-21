use crate::cli::error::CliError;
use crate::web;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn run_server() -> Result<(), CliError> {
    let app = web::routes().await;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| CliError::IoError(e))?;
    
    axum::serve(listener, app)
        .await
        .map_err(|e| CliError::IoError(e))?;

    Ok(())
}

pub fn run_dev() -> Result<(), CliError> {
    println!("Starting development server...");
    // TODO: Implement dev server with hot reload
    Ok(())
} 