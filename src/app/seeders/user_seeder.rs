use crate::app::models::User;
use crate::framework::database::seeder::{DatabaseSeeder, Seeder};
use crate::framework::database::DatabaseError;
use crate::framework::database::factory::Factory;
use async_trait::async_trait;
use once_cell::sync::Lazy;

#[derive(Clone)]
pub struct UserSeeder;

static SEEDER: Lazy<()> = Lazy::new(|| {
    println!("Initializing UserSeeder...");
    let seeder = UserSeeder;
    println!("Registering UserSeeder with DatabaseSeeder...");
    DatabaseSeeder::register(Box::new(seeder));
    println!("UserSeeder registered successfully");
});

pub fn initialize() {
    Lazy::force(&SEEDER);
}

#[async_trait]
impl Seeder for UserSeeder {
    async fn run(&self) -> Result<(), DatabaseError> {
        println!("Running UserSeeder...");
        let users = User::create_many(10).await?;
        println!("Created {} users successfully", users.len());
        Ok(())
    }
} 