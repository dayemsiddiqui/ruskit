pub mod models;
pub mod entities;
pub mod controllers;
pub mod factories;
pub mod seeders;
pub mod dtos;

/// Re-exports commonly used items for better developer experience
pub mod prelude {
    // Re-export framework database traits and types
    pub use crate::framework::database::model::{Model, ValidationRules, HasMany, HasOne, BelongsTo, Field, ModelValidation, Rules, Validate};
    pub use crate::framework::database::query_builder::QueryBuilder;
    pub use crate::framework::database::DatabaseError;
    pub use crate::framework::database::migration::Migration;
    
    // Re-export validation related items
    pub use validator::ValidationError;
    pub use rustavel_derive::GenerateValidationFields;
    
    // Re-export common serialization traits
    pub use serde::{Serialize, Deserialize};
    pub use sqlx::FromRow;
    
    // Re-export async traits
    pub use async_trait::async_trait;

    // Re-export commonly used Axum items
    pub use axum::{
        extract::Path,
        http::StatusCode,
        response::IntoResponse,
        Json,
    };
}

// Re-export commonly used items
pub use controllers::*;
pub use models::*;
pub use factories::*;
pub use seeders::*;

// Initialize all modules that have static initializers
pub fn initialize() {
    println!("Initializing app...");
    println!("Initializing models...");
    models::register_models();
    println!("Models initialized");
    println!("Initializing seeders...");
    // Force initialization of seeders
    seeders::initialize();
    // Ensure the seeder is registered
    seeders::user_seeder::initialize();
    println!("Seeders initialized");
    println!("App initialized");
} 