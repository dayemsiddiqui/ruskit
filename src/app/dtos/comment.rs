use crate::framework::prelude::*;
use crate::app::entities::Comment;
use validator::Validate;

#[derive(Serialize)]
pub struct CommentResponse {
    pub id: i64,
    // Add your fields here
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Deserialize, Validate)]
pub struct CreateCommentRequest {
    // Add your validation fields here
}

#[derive(Serialize)]
pub struct CommentListResponse {
    pub data: Vec<CommentResponse>,
}

impl From<Vec<Comment>> for CommentListResponse {
    fn from(items: Vec<Comment>) -> Self {
        Self {
            data: items.into_iter().map(CommentResponse::from).collect(),
        }
    }
}

impl From<Comment> for CommentResponse {
    fn from(item: Comment) -> Self {
        Self {
            id: item.id,
            // Map your fields here
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

impl From<CreateCommentRequest> for Comment {
    fn from(req: CreateCommentRequest) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
            
        Self {
            id: 0,
            // Map your fields here
            created_at: now,
            updated_at: now,
        }
    }
}