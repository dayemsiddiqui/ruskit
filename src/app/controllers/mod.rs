pub mod pages;
pub mod user_controller;
pub mod docs_controller;
pub mod inertia_controller;
pub mod posts_controller;
pub mod auth_controller;

pub use pages::*;
pub use user_controller::*;
pub use docs_controller::*;
pub use inertia_controller::*;
pub use posts_controller::routes as posts_routes;
pub use auth_controller::*;
