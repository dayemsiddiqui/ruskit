use axum::{
    Router,
    routing::{get, post},
    response::Json,
    extract::Path,
};
use askama::Template;
use askama_axum::{Response, IntoResponse};
use serde_json::{json, Value};

use crate::framework::middleware::{
    WithMiddleware,
    presets::{Cors, TrimStrings},
};
use crate::bootstrap::app::bootstrap;

// Define templates
#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

// Route handlers
async fn home() -> Response {
    let template = HomeTemplate;
    template.into_response()
}   

// Route handlers
async fn index() -> Json<Value> {
    Json(json!({ "message": "Welcome to Rustavel!" }))
}

async fn users_index() -> Json<Value> {
    Json(json!({ "message": "List of users" }))
}

async fn users_show(Path(id): Path<String>) -> Json<Value> {
    Json(json!({
        "message": "Show user details",
        "id": id
    }))
}

async fn users_store() -> Json<Value> {
    Json(json!({ "message": "Create new user" }))
}

// Define routes with middleware
pub async fn routes() -> Router {
    bootstrap().await;
    
    let router = Router::new()
        .route(
            "/", 
            get(home).middleware(Cors::new("http://specific.example.com"))
        );

    let api_router = Router::new()
        .route("/", get(index))
        .route(
            "/users", 
            get(users_index)
                .middleware(TrimStrings::new())
                .middleware(Cors::new("http://users.example.com"))
        )
        .route("/users/{id}", get(users_show))
        .route(
            "/users", 
            post(users_store).middleware(TrimStrings::new())
        );

    router.nest("/api", api_router)
} 