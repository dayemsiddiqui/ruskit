use crate::framework::prelude::*;
use sea_orm::DatabaseConnection;
use axum_inertia::vite;
use axum_login::{
    tower_sessions::{MemoryStore, SessionManagerLayer, Expiry},
    AuthManagerLayer, AuthManagerLayerBuilder,
};
use tower_sessions::cookie::time::Duration as CookieDuration;
use crate::app::services::auth_service::Backend;

pub struct AppConfig {
    pub db: DatabaseConnection,
    pub inertia: InertiaConfig,
    pub auth_layer: AuthManagerLayer<Backend, MemoryStore>,
}

impl AppConfig {
    pub async fn new(db: DatabaseConnection) -> Self {
        let inertia_config = vite::Development::default()
            .port(3000)
            .main("resources/js/app.tsx")
            .lang("en")
            .title("Ruskit")
            .react() 
            .into_config();

        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store.clone())
            .with_expiry(Expiry::OnInactivity(CookieDuration::hours(1)));

        let backend = Backend::new(db.clone());
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        Self {
            db,
            inertia: inertia_config,
            auth_layer,
        }
    }
} 