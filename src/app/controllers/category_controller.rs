use crate::framework::prelude::*;
use crate::app::entities::Category;
use crate::app::dtos::category::{CreateCategoryRequest, CategoryResponse, CategoryListResponse};

/// Category Controller handling all category-related endpoints
pub struct CategoryController {}

impl CategoryController {

    /// Returns a list of categorys
    pub async fn index() -> Json<CategoryListResponse> {
        match Category::all().await {
            Ok(items) => Json(CategoryListResponse::from(items)),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
    }

    /// Returns details for a specific category
    pub async fn show(Path(id): Path<i64>) -> Json<Option<CategoryResponse>> {
        match Category::find(id).await {
            Ok(Some(item)) => Json(Some(CategoryResponse::from(item))),
            Ok(None) => Json(None),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
    }

    /// Creates a new category
    pub async fn store(Json(payload): Json<CreateCategoryRequest>) -> Json<CategoryResponse> {
        let item: Category = payload.into();
        match Category::create(item).await {
            Ok(created) => Json(CategoryResponse::from(created)),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
    }
}