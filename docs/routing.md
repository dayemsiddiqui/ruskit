# Routing in Ruskit

Ruskit provides a simple and expressive routing system built on top of Axum. This guide will help you understand how to define routes and handle HTTP requests in your Ruskit application.

## Basic Routing

The most basic Ruskit routes accept a URI and a closure or function handler:

```rust
Router::new()
    .route("/", get(home))
```

### Available Router Methods

You may register routes for any HTTP verb using the corresponding methods:

```rust
Router::new()
    .route("/users", get(users_index))         // GET
    .route("/users", post(users_store))        // POST
    .route("/users/{id}", put(users_update))   // PUT
    .route("/users/{id}", delete(users_delete)) // DELETE
```

## Route Handlers

Route handlers in Ruskit can return different types of responses:

### Basic String Response
```rust
async fn home() -> &'static str {
    "Welcome to Ruskit!"
}
```

### JSON Response
```rust
async fn users_index() -> Json<Value> {
    Json(json!({ "message": "List of users" }))
}
```

## Route Parameters

### Required Parameters

You can capture route segments using curly braces `{}`:

```rust
// Route definition
.route("/users/{id}", get(users_show))

// Handler
async fn users_show(Path(id): Path<String>) -> Json<Value> {
    Json(json!({
        "message": "Show user details",
        "id": id
    }))
}
```

## API Routes

When building an API, you'll typically want to group related routes under an API prefix:

```rust
Router::new()
    .route("/api/users", get(users_index))
    .route("/api/users/{id}", get(users_show))
    .route("/api/users", post(users_store))
```

## Example Routes File

Here's a complete example of a routes file in Ruskit:

```rust
use axum::{
    Router,
    routing::{get, post},
    response::Json,
    extract::Path,
};
use serde_json::{json, Value};

// Basic route handler
async fn home() -> &'static str {
    "Welcome to Ruskit!"
}

// JSON API handlers
async fn users_index() -> Json<Value> {
    Json(json!({ "message": "List of users" }))
}

async fn users_show(Path(id): Path<String>) -> Json<Value> {
    Json(json!({
        "message": "Show user details",
        "id": id
    }))
}

async fn users_store() -> Json<Value> {
    Json(json!({ "message": "Create new user" }))
}

// Define all routes
pub fn routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/api/users", get(users_index))
        .route("/api/users/{id}", get(users_show))
        .route("/api/users", post(users_store))
}
```

## Request Data Handling

Ruskit provides elegant ways to access different types of request data using Axum's powerful extractors.

### URL Parameters

You can capture URL parameters using curly braces in your route definition:

```rust
use axum::{extract::Path, Router};
use serde::Deserialize;

// Single parameter
async fn show_user(Path(id): Path<String>) -> Response {
    // Access id directly
}

// Multiple parameters
#[derive(Deserialize)]
struct UserPostParams {
    user_id: String,
    post_id: i32,
}

async fn show_user_post(
    Path(params): Path<UserPostParams>
) -> Response {
    // Access params.user_id and params.post_id
}

// In your router
Router::new()
    .route("/users/:id", get(show_user))
    .route("/users/:user_id/posts/:post_id", get(show_user_post))
```

### Query Parameters

Handle query parameters (e.g., `?page=1&limit=10`) using the `Query` extractor:

```rust
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
struct PaginationParams {
    page: Option<u32>,
    limit: Option<u32>,
}

async fn list_users(
    Query(params): Query<PaginationParams>
) -> Response {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    // Use page and limit for pagination
}

// Optional query parameters with defaults
#[derive(Deserialize)]
struct FilterParams {
    #[serde(default = "default_status")]
    status: String,
    #[serde(default)]
    sort_by: String,
}

fn default_status() -> String {
    "active".to_string()
}

async fn filtered_users(
    Query(params): Query<FilterParams>
) -> Response {
    // params.status will be "active" if not provided
    // params.sort_by will be empty string if not provided
}
```

### Request Body

Handle POST, PUT, and PATCH request bodies using the `Json` extractor:

```rust
use axum::{
    extract::Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: i32,
    username: String,
    email: String,
}

async fn create_user(
    Json(payload): Json<CreateUser>
) -> impl IntoResponse {
    // Access payload.username, payload.email, etc.
    Json(UserResponse {
        id: 1,
        username: payload.username,
        email: payload.email,
    })
}

// Handle form data
use axum::extract::Form;

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login(
    Form(data): Form<LoginForm>
) -> Response {
    // Handle form submission
}
```

### Multipart Form Data

Handle file uploads and multipart form data:

```rust
use axum::{
    extract::Multipart,
    response::IntoResponse,
};

async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        
        // Process the uploaded file
        // Example: save to disk, process in memory, etc.
    }
}

// In your router
Router::new()
    .route("/upload", post(upload_file))
```

### Headers and Cookies

Access request headers and cookies:

```rust
use axum::{
    extract::TypedHeader,
    headers::{Authorization, Cookie},
};

async fn authenticated_route(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> Response {
    let token = auth.token();
    let session = cookies.get("session").unwrap_or_default();
    // Handle authentication
}
```

### Combined Extractors

You can combine multiple extractors in a single handler:

```rust
async fn complex_handler(
    Path(id): Path<String>,
    Query(params): Query<PaginationParams>,
    Json(body): Json<CreateUser>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Response {
    // Access all request data in one handler
}
```

### Error Handling

Implement proper error handling for request data:

```rust
use axum::{
    response::IntoResponse,
    http::StatusCode,
};
use serde_json::json;

#[derive(Debug)]
enum ApiError {
    InvalidData(String),
    NotFound,
    Unauthorized,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::InvalidData(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
        };

        Json(json!({
            "error": message
        })).into_response()
    }
}

async fn create_user(
    Json(payload): Json<CreateUser>
) -> Result<impl IntoResponse, ApiError> {
    if payload.username.is_empty() {
        return Err(ApiError::InvalidData("Username is required".to_string()));
    }
    // Process valid data
    Ok(Json(json!({ "message": "User created" })))
}
```

## Best Practices

1. **Type Safety**: Always use strongly typed structs with `Deserialize` for request data
2. **Validation**: Implement validation logic in your data structures
3. **Error Handling**: Use custom error types and proper error responses
4. **Documentation**: Use OpenAPI/Swagger annotations (coming soon)
5. **Testing**: Write tests for your request handlers (documentation coming soon)

## Coming Soon

- OpenAPI/Swagger Integration
- Request Rate Limiting
- Request Validation Middleware
- Request Logging and Monitoring
- API Versioning
- GraphQL Support 