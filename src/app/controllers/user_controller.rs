// Framework imports from prelude
use crate::framework::prelude::*;
// App-specific imports
use crate::app::dtos::user::{CreateUserRequest, UserResponse, UserListResponse};
use crate::app::entities::{user, user::Entity as User};
use axum::{
    extract::Path,
    response::IntoResponse,
    Json,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, QueryOrder, PaginatorTrait};
use std::time::Duration;
use crate::framework::cache::BoxFuture;
use crate::framework::database::DB;

/// User Controller handling all user-related endpoints
pub struct UserController;

impl UserController {
    /// Returns a list of users
    pub async fn index() -> impl IntoResponse {
        let users = Cache::flexible("users:all", Duration::from_secs(300), Duration::from_secs(10), || {
            async move {
                let users = User::find()
                    .order_by(user::Column::Id, sea_orm::Order::Desc)
                    .all(&*DB::connection())
                    .await
                    .unwrap_or_default();
                UserListResponse::from(users)
            }.boxed()
        }).await.unwrap_or_default();

        Json(users)
    }

    /// Returns details for a specific user
    pub async fn show(Path(id): Path<i32>) -> impl IntoResponse {
        let user = Cache::flexible(
            &format!("users:{}", id),
            Duration::from_secs(300),
            Duration::from_secs(10),
            move || async move {
                let user = User::find_by_id(id)
                    .one(&*DB::connection())
                    .await
                    .unwrap_or(None);
                UserResponse::from(user)
            }.boxed()
        ).await.unwrap_or_default();

        Json(user)
    }

    /// Creates a new user
    pub async fn store(Json(payload): Json<CreateUserRequest>) -> impl IntoResponse {
        let user = user::ActiveModel {
            name: Set(payload.name),
            email: Set(payload.email),
            password: Set(String::new()), // This will be set by the auth controller
            role: Set("user".to_string()),
            created_at: Set(chrono::Utc::now().timestamp().to_string()),
            updated_at: Set(chrono::Utc::now().timestamp().to_string()),
            ..Default::default()
        };

        let user = user.insert(&*DB::connection()).await.unwrap();
        
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
        Path(id): Path<i32>,
        Json(payload): Json<CreateUserRequest>,
    ) -> impl IntoResponse {
        let user = User::find_by_id(id)
            .one(&*DB::connection())
            .await
            .unwrap()
            .unwrap();

        let mut user: user::ActiveModel = user.into();
        user.name = Set(payload.name);
        user.email = Set(payload.email);
        user.updated_at = Set(chrono::Utc::now().timestamp().to_string());

        let user = user.update(&*DB::connection()).await.unwrap();
        
        // When updating a user, we should:
        // 1. Update the user's cache
        let response = UserResponse::from(user.clone());
        Cache::put(&format!("users:{}", id), &response, Duration::from_secs(300)).await;
        // 2. Invalidate the users list cache
        Cache::forget("users:all").await;

        Json(response)
    }

    /// Deletes a user
    pub async fn destroy(Path(id): Path<i32>) -> impl IntoResponse {
        let user = User::find_by_id(id)
            .one(&*DB::connection())
            .await
            .unwrap()
            .unwrap();

        let user: user::ActiveModel = user.into();
        user.delete(&*DB::connection()).await.unwrap();
        
        // When deleting a user, we should:
        // 1. Remove the user's cache
        Cache::forget(&format!("users:{}", id)).await;
        // 2. Invalidate the users list cache
        Cache::forget("users:all").await;

        Json(())
    }

    /// Returns user statistics
    pub async fn stats() -> impl IntoResponse {
        let stats = Cache::flexible(
            "user:stats",
            Duration::from_secs(3600), // Cache for 1 hour
            Duration::from_secs(60),   // Grace period of 1 minute
            move || async move {
                let total_users = User::find().count(&*DB::connection()).await.unwrap_or(0);
                json!({
                    "total_users": total_users,
                })
            }.boxed()
        ).await.unwrap_or_default();

        Json(stats)
    }

    /// Increments view count for a user
    pub async fn increment_views(Path(id): Path<i32>) -> impl IntoResponse {
        let views = Cache::increment(&format!("users:{}:views", id), 1).await;
        Json(json!({ "views": views }))
    }
} 