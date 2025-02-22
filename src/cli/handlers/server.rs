use std::net::SocketAddr;
use tokio::net::TcpListener;
use crate::cli::error::CliError;
use crate::web;
use crate::web::AppState;
use crate::bootstrap::app::bootstrap;
use axum_inertia::{InertiaConfig, vite};

pub async fn start_server() -> Result<(), CliError> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(&addr).await?;
    println!("Server running on {}", addr);

    // Initialize the application and get the database connection
    let db = bootstrap().await.map_err(|e| CliError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
    
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
    let service = app.into_make_service_with_connect_info::<SocketAddr>();
    
    axum::serve(listener, service)
        .await
        .map_err(|e| CliError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    Ok(())
}

pub fn run_dev() -> Result<(), CliError> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            start_server().await
        })
} 