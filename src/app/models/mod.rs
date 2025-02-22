use crate::framework::database::model::Model;
use crate::app::entities::User;

// This file will be populated with models as they are created
mod user;

pub use user::*;

// Register all models
pub fn register_models() {
    println!("Registering models...");
    println!("Models registered");
}