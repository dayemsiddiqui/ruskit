// Framework imports from prelude
use crate::app::prelude::*;
// App-specific imports
use crate::app::entities::{Post, User};
use crate::app::dtos::post::{CreatePostRequest, PostResponse, PostListResponse};

/// Post Controller handling all post-related endpoints
pub struct PostController {}

impl PostController {
    /// Returns a list of posts
    pub async fn index() -> impl IntoResponse {
        match Post::all().await {
            Ok(posts) => Json(PostListResponse::from(posts)),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
    }

    /// Returns details for a specific post
    pub async fn show(Path(id): Path<i64>) -> impl IntoResponse {
        match Post::find(id).await {
            Ok(Some(post)) => Json(Some(PostResponse::from(post))),
            Ok(None) => Json(None::<PostResponse>),
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

    pub async fn recent() -> impl IntoResponse {
        match Post::recent(10).await {
            Ok(posts) => Json(PostListResponse::from(posts)),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
    }
}