use axum::{
    Router,
    routing::{get, post},
};

use crate::bootstrap::app::bootstrap;
use crate::app::controllers::{
    user_controller::UserController,
    docs_controller::DocsController,
    inertia_controller::InertiaController,
    pages::landing,
    posts_routes,
};
use axum_inertia::vite;
use sea_orm::DatabaseConnection;
use axum::extract::FromRef;

#[derive(Clone)]
pub struct AppState {
    pub inertia: axum_inertia::InertiaConfig,
    pub db: DatabaseConnection,
}

impl FromRef<AppState> for axum_inertia::InertiaConfig {
    fn from_ref(state: &AppState) -> Self {
        state.inertia.clone()
    }
}

// Define routes with middleware
pub async fn routes() -> Router {
    // Initialize the application
    let db = if let Ok(db) = bootstrap().await {
        db
    } else {
        eprintln!("Failed to bootstrap application");
        std::process::exit(1);
    };

    let inertia_config = vite::Development::default()
        .port(3000)
        .main("resources/js/app.tsx")
        .lang("en")
        .title("Ruskit")
        .react() 
        .into_config();
    
    let app_state = AppState {
        inertia: inertia_config,
        db: db.clone(),
    };

    let inertia_router = Router::new()
        .route("/", get(landing))
        .route("/about", get(InertiaController::about))
        .route("/docs", get(DocsController::index))
        .route("/docs/:page", get(DocsController::show))
        .route("/posts", get(InertiaController::posts_index))
        .route("/posts/create", get(InertiaController::posts_create))
        .route("/posts/:id", get(InertiaController::posts_show))
        .route("/posts/:id/edit", get(InertiaController::posts_edit))
        .with_state(app_state);

    let api_router = Router::new()
        .route("/users", get(UserController::index))
        .route("/users/{id}", get(UserController::show))
        .route("/users", post(UserController::store))
        .merge(posts_routes())
        .with_state(db);

    Router::new()
        .nest("/", inertia_router)
        .nest("/api", api_router)
} 