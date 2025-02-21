use crate::framework::prelude::*;
use crate::framework::database::seeder::{Seeder, DatabaseSeeder};
use once_cell::sync::Lazy;

pub struct PostSeeder;

#[async_trait]
impl Seeder for PostSeeder {
    async fn run(&self) -> Result<(), DatabaseError> {
        // TODO: Add your seeding logic here
        Ok(())
    }
}

static SEEDER: Lazy<()> = Lazy::new(|| {
    DatabaseSeeder::register(Box::new(PostSeeder));
});