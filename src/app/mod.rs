pub mod models;
pub mod entities;
pub mod controllers;
pub mod dtos;

// Re-export commonly used items
pub use controllers::*;
pub use models::*;

// Initialize all modules that have static initializers
pub fn initialize() {
    println!("Initializing app...");
    println!("Initializing models...");
    models::register_models();
    println!("Models initialized");
    println!("App initialized");
} 