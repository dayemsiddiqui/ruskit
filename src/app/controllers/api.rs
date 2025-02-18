use axum::{
    response::Json,
    extract::Path,
};
use serde_json::{json, Value};
use crate::app::models::User;
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the welcome message for the API
pub async fn index() -> Json<Value> {
    Json(json!({ "message": "Welcome to the API!" }))
}

/// Returns a list of users
pub async fn users_index() -> Json<Value> {
    match User::all().await {
        Ok(users) => Json(json!({ "data": users })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

/// Returns details for a specific user
pub async fn users_show(Path(id): Path<i64>) -> Json<Value> {
    match User::find(id).await {
        Ok(Some(user)) => Json(json!({ "data": user })),
        Ok(None) => Json(json!({ "error": "User not found" })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

/// Creates a new user
pub async fn users_store(Json(payload): Json<Value>) -> Json<Value> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
        
    let user_data = json!({
        "name": payload.get("name").and_then(|v| v.as_str()).unwrap_or(""),
        "email": payload.get("email").and_then(|v| v.as_str()).unwrap_or(""),
        "created_at": now,
        "updated_at": now
    });
    
    match User::create(user_data).await {
        Ok(user) => Json(json!({ "data": user })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
} 