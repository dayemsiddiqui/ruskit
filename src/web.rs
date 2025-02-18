use axum::{
    Router,
    routing::{get, post},
};
use crate::framework::middleware::{
    WithMiddleware,
    presets::{Cors, TrimStrings},
};
use crate::bootstrap::app::bootstrap;
use crate::app::controllers::{
    pages::{landing, about},
    user_controller::UserController,
    docs_controller::DocsController,
};

// Define routes with middleware
pub async fn routes() -> Router {
    // Initialize the application
    if let Err(e) = bootstrap().await {
        eprintln!("Failed to bootstrap application: {}", e);
        std::process::exit(1);
    }
    
    let router = Router::new()
        .route(
            "/", 
            get(landing).middleware(Cors::new("http://specific.example.com"))
        )
        .route("/about", get(about))
        .route("/docs", get(DocsController::index))
        .route("/docs/:page", get(DocsController::show));

    let api_router = Router::new()
        .route(
            "/users", 
            get(UserController::index)
                .middleware(TrimStrings::new())
                .middleware(Cors::new("http://users.example.com"))
        )
        .route("/users/{id}", get(UserController::show))
        .route(
            "/users", 
            post(UserController::store).middleware(TrimStrings::new())
        );

    router.nest("/api", api_router)
} 