use std::net::SocketAddr;
use ruskit::web;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Get the router
    let app = web::routes().await;

    // Set up the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Server running on http://{}", addr);

    // Create the listener
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
