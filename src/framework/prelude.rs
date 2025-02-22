// Re-export common serialization traits
pub use serde::{Serialize, Deserialize};

// Re-export async traits
pub use async_trait::async_trait;

// Re-export commonly used Axum items
pub use axum::{
    extract::{Path, FromRef},
    http::{StatusCode, Request},
    response::{IntoResponse, Response, Redirect},
    routing::{get, post},
    middleware::{self, Next, from_fn},
    Router,
    Json,
    body::Body,
};

// Re-export Inertia related items
pub use axum_inertia::{Inertia, InertiaConfig};

// Re-export authentication related items
pub use axum_login::{
    AuthManagerLayerBuilder,
    AuthSession,
    tower_sessions::{MemoryStore, SessionManagerLayer, Expiry},
};

// Re-export tower related items
pub use tower_sessions::cookie::time::Duration as CookieDuration;
pub use tower_http::services::ServeDir;

pub use crate::framework::views::*;
pub use crate::framework::middleware::*;
pub use crate::framework::routing::*;
pub use crate::framework::inertia::*; 