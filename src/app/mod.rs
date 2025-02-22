pub mod console;
pub mod controllers;
pub mod models;
pub mod entities;
pub mod dtos;
pub mod services;
pub mod middleware;
pub mod jobs;

// Re-export commonly used items
pub use controllers::*;
pub use middleware::*;
pub use dtos::*;
pub use entities::*;
pub use models::*;
pub use jobs::*;

// Initialize all modules
pub fn initialize() {
    println!("Initializing app...");
    println!("App initialized");
}
