use axum::{
    Router,
    routing::{get, post},
};
use axum_login::{
    AuthManagerLayerBuilder,
    tower_sessions::{MemoryStore, SessionManagerLayer, Expiry},
};
use sea_orm::DatabaseConnection;
use tower_http::services::ServeDir;
use crate::{
    app::controllers::{auth_controller::AuthController, user_controller::UserController},
    app::services::auth_service::Backend,
};
use axum::extract::FromRef;
use axum_inertia::InertiaConfig;
use tower_sessions::cookie::time::Duration as CookieDuration;
use crate::bootstrap::app::bootstrap;
use crate::app::{
    controllers::{
        docs_controller::DocsController,
        inertia_controller::InertiaController,
        pages::landing,
    },
};
use axum_inertia::vite;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub inertia: InertiaConfig,
}

impl FromRef<AppState> for DatabaseConnection {
    fn from_ref(state: &AppState) -> DatabaseConnection {
        state.db.clone()
    }
}

impl FromRef<AppState> for InertiaConfig {
    fn from_ref(state: &AppState) -> InertiaConfig {
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

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(CookieDuration::hours(1)));

    let backend = Backend::new(db);
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    // API routes
    let api_router = Router::new()
        .route("/me", get(AuthController::me))
        .route("/login", post(AuthController::login))
        .route("/register", post(AuthController::register))
        .route("/logout", post(AuthController::logout))
        .route("/users", get(UserController::index))
        .route("/users/:id", get(UserController::show));

    // Inertia page routes
    let inertia_router = Router::new()
        .route("/", get(landing))
        .route("/about", get(InertiaController::about))
        .route("/docs", get(DocsController::index))
        .route("/docs/:page", get(DocsController::show))
        .route("/posts", get(InertiaController::posts_index))
        .route("/posts/create", get(InertiaController::posts_create))
        .route("/posts/:id", get(InertiaController::posts_show))
        .route("/posts/:id/edit", get(InertiaController::posts_edit))
        .route("/login", get(InertiaController::login))
        .route("/register", get(InertiaController::register))
        .route("/dashboard", get(InertiaController::dashboard));

    Router::new()
        .nest("/api", api_router)
        .merge(inertia_router)
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(auth_layer)
        .with_state(app_state)
} 