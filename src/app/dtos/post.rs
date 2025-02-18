use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::app::models::Post;

#[derive(Serialize)]
pub struct PostResponse {
    pub id: i64,
    // Add your fields here
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Deserialize, Validate)]
pub struct CreatePostRequest {
    // Add your validation fields here
}

#[derive(Serialize)]
pub struct PostListResponse {
    pub data: Vec<PostResponse>,
}

impl From<Vec<Post>> for PostListResponse {
    fn from(items: Vec<Post>) -> Self {
        Self {
            data: items.into_iter().map(PostResponse::from).collect(),
        }
    }
}

impl From<Post> for PostResponse {
    fn from(item: Post) -> Self {
        Self {
            id: item.id,
            // Map your fields here
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

impl From<CreatePostRequest> for Post {
    fn from(req: CreatePostRequest) -> Self {
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