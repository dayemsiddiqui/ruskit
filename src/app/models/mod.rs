use crate::framework::database::model::Model;

// This file will be populated with models as they are created
mod user;
pub use user::User;

// Register all models
pub fn register_models() {
    User::register();
}