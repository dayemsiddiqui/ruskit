use crate::framework::prelude::*;
use crate::app::entities::Comment;
use crate::app::dtos::comment::{CreateCommentRequest, CommentResponse, CommentListResponse};

/// Comment Controller handling all comment-related endpoints
pub struct CommentController {}

impl CommentController {

    /// Returns a list of comments
    pub async fn index() -> Json<CommentListResponse> {
        match Comment::all().await {
            Ok(items) => Json(CommentListResponse::from(items)),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
    }

    /// Returns details for a specific comment
    pub async fn show(Path(id): Path<i64>) -> Json<Option<CommentResponse>> {
        match Comment::find(id).await {
            Ok(Some(item)) => Json(Some(CommentResponse::from(item))),
            Ok(None) => Json(None),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
    }

    /// Creates a new comment
    pub async fn store(Json(payload): Json<CreateCommentRequest>) -> Json<CommentResponse> {
        let item: Comment = payload.into();
        match Comment::create(item).await {
            Ok(created) => Json(CommentResponse::from(created)),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
    }
}