use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::app::models::Post;

#[derive(Serialize)]
pub struct PostResponse {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Deserialize, Validate)]
pub struct CreatePostRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
    pub user_id: i64,
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
            user_id: item.user_id,
            title: item.title,
            content: item.content,
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
            user_id: req.user_id,
            title: req.title,
            content: req.content,
            created_at: now,
            updated_at: now,
        }
    }
}