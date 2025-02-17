use axum::{
    Router,
    routing::{get, post},
    response::Json,
    extract::Path,
};
use serde_json::{json, Value};


async fn home() -> &'static str {
    "Welcome to Rustavel!"
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

// Define routes
pub fn routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/api", get(index))
        .route("/api/users", get(users_index))
        .route("/api/users/{id}", get(users_show))
        .route("/api/users", post(users_store))
} 