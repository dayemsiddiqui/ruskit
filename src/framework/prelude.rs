// Re-export framework database traits and types
pub use crate::framework::database::model::{Model, ValidationRules, HasMany, HasOne, BelongsTo, Field, ModelValidation, Rules, Validate};
pub use crate::framework::database::query_builder::QueryBuilder;
pub use crate::framework::database::DatabaseError;
pub use crate::framework::database::migration::Migration;
pub use crate::framework::database::factory::Factory;

// Re-export validation related items
pub use validator::ValidationError;
pub use rustavel_derive::GenerateValidationFields;

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