use crate::framework::database::model::Model;
use crate::framework::database::migration::Migration;

// This file will be populated with models as they are created
mod user;
pub use user::User;

// Register all models
pub fn register_models() {
    User::register();
}

// Get all migrations from all models
pub fn get_all_model_migrations() -> Vec<Migration> {
    let mut migrations = Vec::new();
    migrations.extend(User::migrations());
    migrations
}
mod post;
pub use post::Post;