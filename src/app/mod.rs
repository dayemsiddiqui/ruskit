pub mod controllers;
pub mod dtos;

// Re-export commonly used items
pub use controllers::*;

// Initialize all modules
pub fn initialize() {
    println!("Initializing app...");
    println!("App initialized");
}
