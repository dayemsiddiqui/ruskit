// Framework imports from prelude
use crate::framework::prelude::*;
// App-specific imports
use crate::app::dtos::user::{CreateUserRequest, UserResponse, UserListResponse};
use crate::app::entities::{user, user::Entity as User};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

/// User Controller handling all user-related endpoints
pub struct UserController;

impl UserController {
    /// Returns a list of users
    pub async fn index(State(db): State<DatabaseConnection>) -> impl IntoResponse {
        let users = User::find()
            .all(&db)
            .await
            .unwrap_or_default();

        Json(UserListResponse::from(users))
    }

    /// Returns details for a specific user
    pub async fn show(
        State(db): State<DatabaseConnection>,
        Path(id): Path<i32>
    ) -> impl IntoResponse {
        let user = User::find_by_id(id)
            .one(&db)
            .await
            .unwrap()
            .map(UserResponse::from);

        Json(user)
    }

    /// Creates a new user
    pub async fn store(
        State(db): State<DatabaseConnection>,
        Json(payload): Json<CreateUserRequest>
    ) -> Json<UserResponse> {
        let user = user::ActiveModel {
            name: Set(payload.name),
            email: Set(payload.email),
            ..Default::default()
        };

        let user = user.insert(&db).await.unwrap();
        Json(UserResponse::from(user))
    }

    pub async fn recent(State(db): State<DatabaseConnection>) -> impl IntoResponse {
        let users = User::find()
            .all(&db)
            .await
            .unwrap_or_default();

        Json(UserListResponse::from(users))
    }
} 