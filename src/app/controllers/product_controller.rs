use crate::framework::prelude::*;
use crate::app::entities::Product;
use crate::app::dtos::product::{CreateProductRequest, ProductResponse, ProductListResponse};

/// Product Controller handling all product-related endpoints
pub struct ProductController {}

impl ProductController {

    /// Returns a list of products
    pub async fn index() -> Json<ProductListResponse> {
        match Product::all().await {
            Ok(items) => Json(ProductListResponse::from(items)),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
    }

    /// Returns details for a specific product
    pub async fn show(Path(id): Path<i64>) -> Json<Option<ProductResponse>> {
        match Product::find(id).await {
            Ok(Some(item)) => Json(Some(ProductResponse::from(item))),
            Ok(None) => Json(None),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
    }

    /// Creates a new product
    pub async fn store(Json(payload): Json<CreateProductRequest>) -> Json<ProductResponse> {
        let item: Product = payload.into();
        match Product::create(item).await {
            Ok(created) => Json(ProductResponse::from(created)),
            Err(e) => panic!("Database error: {}", e), // In a real app, use proper error handling
        }
    }
}