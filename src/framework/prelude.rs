// Re-export common serialization traits
pub use serde::{Serialize, Deserialize};

// Re-export async traits
pub use async_trait::async_trait;

// Re-export commonly used Axum items
pub use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

// Re-export Inertia
pub use axum_inertia::Inertia;

pub use crate::framework::views::*;
pub use crate::framework::middleware::*;
pub use crate::framework::routing::*;
pub use crate::framework::inertia::*; 