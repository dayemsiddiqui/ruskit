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
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, QueryOrder, PaginatorTrait};
use std::time::Duration;
use serde_json::Value;

/// User Controller handling all user-related endpoints
pub struct UserController;

impl UserController {
    /// Returns a list of users
    pub async fn index(State(db): State<DatabaseConnection>) -> impl IntoResponse {
        // Try to get users from cache first
        if let Some(users) = Cache::get::<UserListResponse>("users:all").await {
            return Json(users);
        }

        // If not in cache, get from database
        let users = User::find()
            .order_by(user::Column::Id, sea_orm::Order::Desc)
            .all(&db)
            .await
            .unwrap_or_default();

        let response = UserListResponse::from(users);

        // Cache the result for 5 minutes
        Cache::put("users:all", &response, Duration::from_secs(300)).await;

        Json(response)
    }

    /// Returns details for a specific user
    pub async fn show(
        State(db): State<DatabaseConnection>,
        Path(id): Path<i32>
    ) -> impl IntoResponse {
        let user = User::find_by_id(id).one(&db).await.unwrap_or(None);
        let user_response = UserResponse::from(user);
        
        // Cache the response for future requests
        Cache::put(
            &format!("users:{}", id),
            &user_response,
            Duration::from_secs(300),
        ).await;

        Json(user_response)
    }

    /// Creates a new user
    pub async fn store(
        State(db): State<DatabaseConnection>,
        Json(payload): Json<CreateUserRequest>
    ) -> impl IntoResponse {
        let user = user::ActiveModel {
            name: Set(payload.name),
            email: Set(payload.email),
            password: Set(String::new()), // This will be set by the auth controller
            role: Set("user".to_string()),
            created_at: Set(chrono::Utc::now().timestamp().to_string()),
            updated_at: Set(chrono::Utc::now().timestamp().to_string()),
            ..Default::default()
        };

        let user = user.insert(&db).await.unwrap();
        
        // When creating a new user, we should:
        // 1. Cache the new user
        let response = UserResponse::from(user.clone());
        Cache::forever(&format!("users:{}", user.id), &response).await;
        // 2. Invalidate the users list cache
        Cache::forget("users:all").await;

        Json(response)
    }

    /// Updates a user
    pub async fn update(
        State(db): State<DatabaseConnection>,
        Path(id): Path<i32>,
        Json(payload): Json<CreateUserRequest>,
    ) -> impl IntoResponse {
        let user = User::find_by_id(id)
            .one(&db)
            .await
            .unwrap()
            .unwrap();

        let mut user: user::ActiveModel = user.into();
        user.name = Set(payload.name);
        user.email = Set(payload.email);
        user.updated_at = Set(chrono::Utc::now().timestamp().to_string());

        let user = user.update(&db).await.unwrap();
        
        // When updating a user, we should:
        // 1. Update the user's cache
        let response = UserResponse::from(user.clone());
        Cache::put(&format!("users:{}", id), &response, Duration::from_secs(300)).await;
        // 2. Invalidate the users list cache
        Cache::forget("users:all").await;

        Json(response)
    }

    /// Deletes a user
    pub async fn destroy(
        State(db): State<DatabaseConnection>,
        Path(id): Path<i32>,
    ) -> impl IntoResponse {
        let user = User::find_by_id(id)
            .one(&db)
            .await
            .unwrap()
            .unwrap();

        let user: user::ActiveModel = user.into();
        user.delete(&db).await.unwrap();
        
        // When deleting a user, we should:
        // 1. Remove the user's cache
        Cache::forget(&format!("users:{}", id)).await;
        // 2. Invalidate the users list cache
        Cache::forget("users:all").await;

        Json(())
    }

    /// Returns user statistics
    pub async fn stats(State(db): State<DatabaseConnection>) -> impl IntoResponse {
        let total_users = User::find().count(&db).await.unwrap_or(0);
        let stats = json!({
            "total_users": total_users,
        });
        
        // Cache the stats forever
        Cache::forever("user:stats", &stats).await;

        Json(stats)
    }

    /// Increments view count for a user
    pub async fn increment_views(Path(id): Path<i32>) -> impl IntoResponse {
        let views = Cache::increment(&format!("users:{}:views", id), 1).await;
        Json(json!({ "views": views }))
    }
} 