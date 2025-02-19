use std::net::SocketAddr;
use ruskit::web;
use tokio::net::TcpListener;
use ts_rs::TS;

#[tokio::main]
async fn main() {
    // Ensure the types directory exists and generate TypeScript types
    std::fs::create_dir_all("resources/js/types").unwrap();
    
    // Generate TypeScript types
    ruskit::app::dtos::about::AboutPageProps::export_to("resources/js/types/generated.ts").unwrap();

    // Get the router
    let app = web::routes().await;

    // Set up the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Server running on http://{}", addr);

    // Create the listener
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
