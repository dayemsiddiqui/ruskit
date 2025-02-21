use crate::framework::prelude::*;
use crate::app::entities::Product;
use validator::Validate;

#[derive(Serialize)]
pub struct ProductResponse {
    pub id: i64,
    // Add your fields here
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Deserialize, Validate)]
pub struct CreateProductRequest {
    // Add your validation fields here
}

#[derive(Serialize)]
pub struct ProductListResponse {
    pub data: Vec<ProductResponse>,
}

impl From<Vec<Product>> for ProductListResponse {
    fn from(items: Vec<Product>) -> Self {
        Self {
            data: items.into_iter().map(ProductResponse::from).collect(),
        }
    }
}

impl From<Product> for ProductResponse {
    fn from(item: Product) -> Self {
        Self {
            id: item.id,
            // Map your fields here
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

impl From<CreateProductRequest> for Product {
    fn from(_req: CreateProductRequest) -> Self {
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