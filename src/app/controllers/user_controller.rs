use axum::{
    response::Json,
    extract::Path,
};
use crate::app::models::User;
use crate::app::dtos::{CreateUserRequest, UserResponse, UserListResponse};
use crate::framework::database::model::Model;

/// User Controller handling all user-related endpoints
pub struct UserController;

impl UserController {
    /// Returns a list of users
    pub async fn index() -> Json<UserListResponse> {
        match User::all().await {
            Ok(users) => Json(UserListResponse::from(users)),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
    }

    /// Returns details for a specific user
    pub async fn show(Path(id): Path<i64>) -> Json<Option<UserResponse>> {
        match User::find(id).await {
            Ok(Some(user)) => Json(Some(UserResponse::from(user))),
            Ok(None) => Json(None),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
    }

    /// Creates a new user
    pub async fn store(Json(payload): Json<CreateUserRequest>) -> Json<UserResponse> {
        let user: User = payload.into();
        match User::create(user).await {
            Ok(user) => Json(UserResponse::from(user)),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
    }
} 