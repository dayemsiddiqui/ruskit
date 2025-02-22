pub mod console;
pub mod controllers;
pub mod models;
pub mod entities;
pub mod dtos;
pub mod services;
pub mod middleware;

// Re-export commonly used items
pub use controllers::*;
pub use middleware::*;

// Initialize all modules
pub fn initialize() {
    println!("Initializing app...");
    println!("App initialized");
}
