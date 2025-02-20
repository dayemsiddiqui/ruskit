use crate::app::entities::User;
use crate::framework::database::model::Model;
use crate::framework::database::factory::Factory;

pub fn initialize() {
    println!("Initializing user seeder...");
}

pub async fn run() {
    println!("Running user seeder...");
    for _ in 0..10 {
        let user = User::definition();
        User::create(user).await.unwrap();
    }
    println!("User seeder completed");
} 