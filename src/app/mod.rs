pub mod controllers;
pub mod dtos;
pub mod entities;
pub mod services;

// Re-export commonly used items
pub use controllers::*;

// Initialize all modules
pub fn initialize() {
    println!("Initializing app...");
    println!("App initialized");
}
