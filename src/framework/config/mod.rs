use crate::framework::prelude::*;
use sea_orm::DatabaseConnection;
use axum_inertia::vite;
use axum_login::{
    tower_sessions::{MemoryStore, SessionManagerLayer, Expiry},
    AuthManagerLayer, AuthManagerLayerBuilder,
};
use tower_sessions::cookie::time::Duration as CookieDuration;
use crate::app::services::auth_service::Backend;

/// Configuration for the entire application
#[derive(Clone)]
pub struct AppConfig {
    /// Database connection
    pub db: DatabaseConnection,
    /// Inertia.js configuration
    pub inertia: InertiaConfig,
    /// Authentication layer configuration
    pub auth_layer: AuthManagerLayer<Backend, MemoryStore>,
}

impl AppConfig {
    /// Create a new application configuration
    pub async fn new(db: DatabaseConnection) -> Self {
        let inertia_config = Self::default_inertia_config();
        let auth_layer = Self::default_auth_layer(db.clone()).await;

        Self {
            db,
            inertia: inertia_config,
            auth_layer,
        }
    }

    /// Create default Inertia.js configuration
    pub fn default_inertia_config() -> InertiaConfig {
        vite::Development::default()
            .port(3000)
            .main("resources/js/app.tsx")
            .lang("en")
            .title("Ruskit")
            .react() 
            .into_config()
    }

    /// Create default authentication layer configuration
    pub async fn default_auth_layer(db: DatabaseConnection) -> AuthManagerLayer<Backend, MemoryStore> {
        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store.clone())
            .with_expiry(Expiry::OnInactivity(CookieDuration::hours(1)));

        let backend = Backend::new(db);
        AuthManagerLayerBuilder::new(backend, session_layer).build()
    }
} 