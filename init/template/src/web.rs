use axum::{
    Router,
    routing::get,
};
use crate::app::controllers;

pub async fn routes() -> Router {
    Router::new()
        // Static routes
        .route("/", get(controllers::index))
        
        // API routes
        .nest("/api", api_routes())
}

fn api_routes() -> Router {
    Router::new()
        // Add your API routes here
        // Example:
        // .route("/users", get(controllers::users::index))
        // .route("/users/:id", get(controllers::users::show))
} 