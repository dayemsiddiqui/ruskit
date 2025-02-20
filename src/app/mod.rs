pub mod models;
pub mod entities;
pub mod controllers;
pub mod seeders;
pub mod dtos;

// Re-export commonly used items
pub use controllers::*;
pub use models::*;
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