use crate::framework::database::seeder::{Seeder, DatabaseSeeder};
use crate::framework::database::DatabaseError;
use crate::app::models::User;
use crate::framework::database::factory::Factory;
use async_trait::async_trait;
use once_cell::sync::Lazy;

pub struct UserSeeder;

#[async_trait]
impl Seeder for UserSeeder {
    async fn run(&self) -> Result<(), DatabaseError> {
        // Create 10 users
        User::create_many(10).await?;
        Ok(())
    }
}

static USER_SEEDER: Lazy<()> = Lazy::new(|| {
    DatabaseSeeder::register(Box::new(UserSeeder));
}); 