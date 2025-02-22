use crate::framework::prelude::*;
use sea_orm::DatabaseConnection;
use crate::bootstrap::app::bootstrap;
use tower_http::services::ServeDir;
use crate::config;
use crate::routes;

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

pub async fn routes() -> Router<AppState> {
    // Initialize the application
    let db = if let Ok(db) = bootstrap().await {
        db
    } else {
        eprintln!("Failed to bootstrap application");
        std::process::exit(1);
    };

    // Set up application configuration
    let config::AppConfig { db, inertia, auth_layer } = config::AppConfig::new(db).await;
    let app_state = AppState { db, inertia };

    Router::new()
        .nest("/api", routes::api_routes())
        .merge(routes::inertia_routes())
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(auth_layer)
        .with_state(app_state)
} 