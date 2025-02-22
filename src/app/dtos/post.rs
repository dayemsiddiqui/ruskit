// DTO for the About page

use ts_export_derive::auto_ts_export;

#[auto_ts_export]
pub struct PostDto {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub created_at: String,
    pub updated_at: String,
}

#[auto_ts_export]
pub struct PostListProps {
    pub posts: Vec<PostDto>
}
