use axum::{
    response::Json,
    extract::Path,
    http::StatusCode,
};
use crate::app::models::Post;
use crate::app::models::User;
use crate::framework::database::model::Model;
use crate::app::dtos::post::{CreatePostRequest, PostResponse, PostListResponse};

/// Post Controller handling all post-related endpoints
pub struct PostController {}

impl PostController {

    /// Returns a list of posts
    pub async fn index() -> Json<PostListResponse> {
        match Post::all().await {
            Ok(items) => Json(PostListResponse::from(items)),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
    }

    /// Returns details for a specific post
    pub async fn show(Path(id): Path<i64>) -> Json<Option<PostResponse>> {
        match Post::find(id).await {
            Ok(Some(item)) => Json(Some(PostResponse::from(item))),
            Ok(None) => Json(None),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
        }

    /// Creates a new post
    pub async fn store(Json(payload): Json<CreatePostRequest>) -> Result<Json<PostResponse>, (StatusCode, String)> {
        let user = match User::find(payload.user_id).await {
            Ok(Some(user)) => user,
            _ => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        };

        let post: Post = payload.into();

        match user.posts().create(post).await {
            Ok(created) => Ok(Json(PostResponse::from(created))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))),
        }
    }
}