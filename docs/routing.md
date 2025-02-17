# Routing in Rustavel

Rustavel provides a simple and expressive routing system built on top of Axum. This guide will help you understand how to define routes and handle HTTP requests in your Rustavel application.

## Basic Routing

The most basic Rustavel routes accept a URI and a closure or function handler:

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

Route handlers in Rustavel can return different types of responses:

### Basic String Response
```rust
async fn home() -> &'static str {
    "Welcome to Rustavel!"
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

Here's a complete example of a routes file in Rustavel:

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
    "Welcome to Rustavel!"
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

## Best Practices

1. **Route Organization**: Keep your routes organized by grouping related endpoints together.
2. **RESTful Naming**: Follow RESTful conventions for your API endpoints:
   - GET `/users` - List users
   - GET `/users/{id}` - Show a specific user
   - POST `/users` - Create a new user
   - PUT `/users/{id}` - Update a user
   - DELETE `/users/{id}` - Delete a user

3. **Response Types**: Be consistent with your response types. For APIs, use JSON responses.

4. **Error Handling**: Implement proper error handling for your routes (documentation coming soon).

## Coming Soon

- Route Groups
- Route Middleware
- Route Model Binding
- Route Caching
- Validation
- Authentication
- Authorization 