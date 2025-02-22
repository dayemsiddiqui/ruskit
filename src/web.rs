use crate::framework::prelude::*;
use sea_orm::DatabaseConnection;
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

pub async fn routes(db: DatabaseConnection) -> Router {
    // Set up application configuration
    let config::AppConfig { inertia, auth_layer, .. } = config::AppConfig::new(db.clone()).await;
    let app_state = AppState { db, inertia };

    Router::new()
        .nest("/api", routes::api_routes())
        .merge(routes::inertia_routes())
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(auth_layer)
        .with_state(app_state)
} 