use crate::framework::prelude::*;
use crate::app::entities::Category;
use validator::Validate;

#[derive(Serialize)]
pub struct CategoryResponse {
    pub id: i64,
    // Add your fields here
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Deserialize, Validate)]
pub struct CreateCategoryRequest {
    // Add your validation fields here
}

#[derive(Serialize)]
pub struct CategoryListResponse {
    pub data: Vec<CategoryResponse>,
}

impl From<Vec<Category>> for CategoryListResponse {
    fn from(items: Vec<Category>) -> Self {
        Self {
            data: items.into_iter().map(CategoryResponse::from).collect(),
        }
    }
}

impl From<Category> for CategoryResponse {
    fn from(item: Category) -> Self {
        Self {
            id: item.id,
            // Map your fields here
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

impl From<CreateCategoryRequest> for Category {
    fn from(_req: CreateCategoryRequest) -> Self {
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