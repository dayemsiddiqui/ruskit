pub mod user_seeder;

pub use user_seeder::*;

// This function is called to ensure all seeders are loaded
pub fn initialize() {
    println!("Loading seeders...");
    // Initialize the user seeder
    println!("Loading user seeder...");
    user_seeder::initialize();
    println!("User seeder loaded");
    println!("All seeders loaded");
}