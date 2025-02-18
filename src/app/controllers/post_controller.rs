use axum::{
    response::Json,
    extract::Path,
};
use crate::app::models::Post;
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
    pub async fn store(Json(payload): Json<CreatePostRequest>) -> Json<PostResponse> {
        let item: Post = payload.into();
        match Post::create(item).await {
            Ok(created) => Json(PostResponse::from(created)),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
    }
}