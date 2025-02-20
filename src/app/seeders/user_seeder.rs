use crate::app::entities::User;
use crate::framework::database::model::Model;

pub fn initialize() {
    println!("Initializing user seeder...");
}

pub async fn run() {
    println!("Running user seeder...");
    User::create_many(10).await.unwrap();
    println!("User seeder completed");
} 