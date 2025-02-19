use ruskit::web;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod app;
mod web;

#[tokio::main]
async fn main() {
    // Initialize the application
    ruskit::app::initialize();

    // Get routes from web.rs
    let app = web::routes().await;

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    // Create the listener
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
} 