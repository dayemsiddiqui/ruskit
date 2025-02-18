# Controllers

Controllers handle incoming HTTP requests and return responses. They are responsible for processing input, interacting with models, and returning appropriate responses.

## Overview

Controllers in Rustavel:
- Handle HTTP requests
- Process input data through DTOs
- Interact with models for data operations
- Return JSON responses
- Follow RESTful conventions

## Creating Controllers

Use the `make:controller` command to generate a new controller:

```bash
cargo kit make:controller Post
```

This creates:
- `src/app/controllers/post_controller.rs`
- Updates `src/app/controllers/mod.rs`

### Generated Structure

```rust
use axum::{
    response::Json,
    extract::Path,
};
use crate::app::models::Post;
use crate::framework::database::model::Model;
use crate::app::dtos::post::{CreatePostRequest, PostResponse, PostListResponse};

pub struct PostController {}

impl PostController {
    pub async fn index() -> Json<PostListResponse> {
        // List all posts
    }

    pub async fn show(Path(id): Path<i64>) -> Json<Option<PostResponse>> {
        // Show single post
    }

    pub async fn store(Json(payload): Json<CreatePostRequest>) -> Json<PostResponse> {
        // Create new post
    }
}
```

## RESTful Methods

Standard RESTful methods in controllers:

### Index - List Resources
```rust
pub async fn index() -> Json<PostListResponse> {
    match Post::all().await {
        Ok(items) => Json(PostListResponse::from(items)),
        Err(e) => panic!("Database error: {}", e),
    }
}
```

### Show - Single Resource
```rust
pub async fn show(Path(id): Path<i64>) -> Json<Option<PostResponse>> {
    match Post::find(id).await {
        Ok(Some(item)) => Json(Some(PostResponse::from(item))),
        Ok(None) => Json(None),
        Err(e) => panic!("Database error: {}", e),
    }
}
```

### Store - Create Resource
```rust
pub async fn store(Json(payload): Json<CreatePostRequest>) -> Json<PostResponse> {
    let item: Post = payload.into();
    match Post::create(item).await {
        Ok(created) => Json(PostResponse::from(created)),
        Err(e) => panic!("Database error: {}", e),
    }
}
```

### Update - Modify Resource
```rust
pub async fn update(
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePostRequest>
) -> Json<PostResponse> {
    match Post::find(id).await {
        Ok(Some(mut item)) => {
            item.update_from(payload);
            match item.save().await {
                Ok(updated) => Json(PostResponse::from(updated)),
                Err(e) => panic!("Database error: {}", e),
            }
        }
        Ok(None) => panic!("Post not found"),
        Err(e) => panic!("Database error: {}", e),
    }
}
```

### Destroy - Delete Resource
```rust
pub async fn destroy(Path(id): Path<i64>) -> Json<()> {
    match Post::find(id).await {
        Ok(Some(item)) => {
            match item.delete().await {
                Ok(_) => Json(()),
                Err(e) => panic!("Database error: {}", e),
            }
        }
        Ok(None) => panic!("Post not found"),
        Err(e) => panic!("Database error: {}", e),
    }
}
```

## Request Handling

### Path Parameters
```rust
use axum::extract::Path;

pub async fn show(Path(id): Path<i64>) -> Json<Option<PostResponse>> {
    // Access id directly
}
```

### Query Parameters
```rust
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListParams {
    page: Option<i32>,
    per_page: Option<i32>,
}

pub async fn index(Query(params): Query<ListParams>) -> Json<PostListResponse> {
    // Access params.page and params.per_page
}
```

### Request Body
```rust
use axum::Json;

pub async fn store(Json(payload): Json<CreatePostRequest>) -> Json<PostResponse> {
    // payload is already deserialized
}
```

## Error Handling

Use Result types for proper error handling:

```rust
use axum::{
    response::Json,
    http::StatusCode,
};
use crate::framework::error::ApiError;

pub async fn show(Path(id): Path<i64>) -> Result<Json<PostResponse>, ApiError> {
    match Post::find(id).await {
        Ok(Some(post)) => Ok(Json(PostResponse::from(post))),
        Ok(None) => Err(ApiError::not_found("Post not found")),
        Err(e) => Err(ApiError::database_error(e)),
    }
}
```

## Middleware

Apply middleware to controller methods:

```rust
use crate::framework::middleware::auth::Auth;

#[derive(Clone)]
pub struct PostController {}

impl PostController {
    pub async fn store(
        Auth(user): Auth,
        Json(payload): Json<CreatePostRequest>
    ) -> Result<Json<PostResponse>, ApiError> {
        // User is authenticated
        let mut post: Post = payload.into();
        post.user_id = user.id;
        // ...
    }
}
```

## Best Practices

1. **Separation of Concerns**
   - Keep controllers thin
   - Move business logic to services
   - Use DTOs for input/output
   - Handle errors appropriately

2. **Resource Naming**
   - Use plural nouns for resource names
   - Follow RESTful conventions
   - Keep names clear and descriptive

3. **Method Naming**
   - Use standard RESTful method names
   - Add custom methods when needed
   - Keep method names descriptive

4. **Error Handling**
   - Use proper error types
   - Return appropriate status codes
   - Provide helpful error messages

5. **Input Validation**
   - Use DTOs for input validation
   - Validate path parameters
   - Check query parameters

6. **Response Format**
   - Use consistent response structures
   - Include appropriate metadata
   - Follow API conventions

7. **Security**
   - Apply authentication middleware
   - Validate user permissions
   - Sanitize input data

## Testing

Example of controller testing:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::helpers::*;

    #[tokio::test]
    async fn test_index_returns_posts() {
        // Setup
        let posts = create_test_posts().await;
        
        // Execute
        let response = PostController::index().await;
        
        // Assert
        match response {
            Json(list) => {
                assert_eq!(list.data.len(), posts.len());
                // Additional assertions...
            }
        }
    }
}
``` 