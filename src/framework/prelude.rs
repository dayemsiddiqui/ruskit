// Re-export framework database types
pub use crate::framework::database::DatabaseError;

// Re-export common serialization traits
pub use serde::{Serialize, Deserialize};
pub use sqlx::{FromRow, sqlite::SqliteRow};

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