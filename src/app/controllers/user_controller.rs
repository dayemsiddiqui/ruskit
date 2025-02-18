use axum::{
    response::Json,
    extract::Path,
};
use serde_json::{json, Value};
use crate::app::models::User;
use crate::framework::database::model::Model;
use std::time::{SystemTime, UNIX_EPOCH};

/// User Controller handling all user-related endpoints
pub struct UserController;

impl UserController {
    /// Returns a list of users
    pub async fn index() -> Json<Value> {
        match User::all().await {
            Ok(users) => Json(json!({ "data": users })),
            Err(e) => Json(json!({ "error": e.to_string() })),
        }
    }

    /// Returns details for a specific user
    pub async fn show(Path(id): Path<i64>) -> Json<Value> {
        match User::find(id).await {
            Ok(Some(user)) => Json(json!({ "data": user })),
            Ok(None) => Json(json!({ "error": "User not found" })),
            Err(e) => Json(json!({ "error": e.to_string() })),
        }
    }

    /// Creates a new user
    pub async fn store(Json(payload): Json<Value>) -> Json<Value> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
            
        let user = User {
            id: 0, // This will be set by the database
            name: payload.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            email: payload.get("email").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            created_at: now,
            updated_at: now,
        };
        
        match User::create(user).await {
            Ok(user) => Json(json!({ "data": user })),
            Err(e) => Json(json!({ "error": e.to_string() })),
        }
    }
} 