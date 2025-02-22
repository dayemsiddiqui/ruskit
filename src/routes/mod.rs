use crate::framework::prelude::*;
use crate::app::controllers::{
    auth_controller::AuthController,
    user_controller::UserController,
    docs_controller::DocsController,
    inertia_controller::InertiaController,
    pages::landing,
};
use crate::app::middleware::require_auth;
use crate::web::AppState;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(AuthController::me))
        .route("/login", post(AuthController::login))
        .route("/register", post(AuthController::register))
        .route("/logout", post(AuthController::logout))
        .route("/users", get(UserController::index))
        .route("/users/:id", get(UserController::show))
}

pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(InertiaController::dashboard))
        .route_layer(from_fn(require_auth))
}

pub fn inertia_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(landing))
        .route("/about", get(InertiaController::about))
        .route("/docs", get(DocsController::index))
        .route("/docs/:page", get(DocsController::show))
        .route("/posts", get(InertiaController::posts_index))
        .route("/posts/create", get(InertiaController::posts_create))
        .route("/posts/:id", get(InertiaController::posts_show))
        .route("/posts/:id/edit", get(InertiaController::posts_edit))
        .route("/login", get(InertiaController::login))
        .route("/register", get(InertiaController::register))
        .merge(protected_routes())
} 