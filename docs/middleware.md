# Middleware in Rustavel

Middleware provides a convenient mechanism for filtering HTTP requests entering your application. For example, Rustavel includes middleware for handling CORS and trimming strings. You may add your own middleware to customize it further.

## Introduction

Middleware acts as a bridge between a request and a response, allowing you to:
- Modify requests before they reach your route handlers
- Modify responses before they are sent back to the client
- Perform actions before or after request handling
- Terminate requests early if certain conditions aren't met

## Using Built-in Middleware

### Available Middleware

Rustavel comes with several built-in middleware components:

#### CORS Middleware
Handles Cross-Origin Resource Sharing headers:

```rust
use rustavel::presets::Cors;

Router::new()
    .route(
        "/api", 
        get(handler).middleware(Cors::new("http://example.com"))
    )
```

Configure CORS with additional options:
```rust
let cors = Cors::new("http://example.com")
    .with_methods("GET, POST, PUT, DELETE")
    .with_headers("Content-Type, Authorization");
```

#### TrimStrings Middleware
Automatically trims string inputs in requests:

```rust
use rustavel::presets::TrimStrings;

Router::new()
    .route(
        "/users", 
        post(create_user).middleware(TrimStrings::new())
    )
```

### Applying Middleware

There are three ways to apply middleware in Rustavel:

#### 1. Route Middleware
Apply middleware to specific routes:

```rust
use rustavel::presets::{Cors, TrimStrings};

Router::new()
    .route(
        "/api/users",
        get(users_index)
            .middleware(TrimStrings::new())
            .middleware(Cors::new("http://example.com"))
    )
```

#### 2. Global Middleware
Apply middleware to every HTTP request:

```rust
// In your bootstrap.rs
pub async fn bootstrap() {
    let app = Application::instance().await;
    let mut app = app.write().await;

    // Configure global middleware
    app.middleware(|stack| {
        stack.add(Middleware::Cors(Cors::new("*")));
        stack.add(Middleware::TrimStrings(TrimStrings::new()));
    }).await;
}
```

#### 3. Middleware Groups
Group middleware for specific sets of routes:

```rust
// Define middleware groups
app.middleware_groups(|groups| {
    groups.push((
        "api",
        vec![
            Middleware::Cors(Cors::new("http://api.example.com")),
            Middleware::TrimStrings(TrimStrings::new())
        ]
    ));
}).await;

// Use middleware group
if let Some(middlewares) = middleware_group("api").await {
    router = router.middlewares(middlewares);
}
```

## Creating Custom Middleware

### Basic Structure

Create a new middleware by implementing a struct with a handle method:

```rust
use axum::{
    middleware::Next,
    response::Response,
    http::Request,
    body::Body,
};

#[derive(Clone)]
pub struct LogRequest;

impl LogRequest {
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn handle(
        &self,
        request: Request<Body>,
        next: Next,
    ) -> Result<Response, Response> {
        println!("Incoming request to: {}", request.uri());
        let response = next.run(request).await;
        println!("Outgoing response");
        Ok(response)
    }
}
```

### Registering Custom Middleware

1. Add your middleware to the internal Middleware enum:

```rust
// In framework/middleware/internal.rs
pub enum Middleware {
    Cors(presets::Cors),
    TrimStrings(presets::TrimStrings),
    LogRequest(LogRequest),  // Add your variant
}
```

2. Implement the From trait:

```rust
impl From<LogRequest> for Middleware {
    fn from(middleware: LogRequest) -> Self {
        Self::LogRequest(middleware)
    }
}
```

### Using Custom Middleware

```rust
use crate::middleware::LogRequest;

Router::new()
    .route("/api", get(handler).middleware(LogRequest::new()))
```

## Best Practices

1. **Order of Middleware**
   - Place middleware that modifies the request before middleware that uses the request
   - Authentication/Authorization middleware should typically run early
   - Logging middleware often works best at the start or end of the chain

2. **Performance Considerations**
   - Keep middleware logic efficient
   - Only use middleware where needed
   - Consider the impact of middleware on response times

3. **Error Handling**
   - Use the Result type to properly handle errors
   - Return appropriate error responses
   - Consider logging middleware errors for debugging

4. **State Management**
   - Use Clone or Arc for sharing state between middleware instances
   - Keep middleware stateless when possible
   - Use configuration structs for middleware that needs configuration

## Middleware Execution Flow

The middleware execution follows this pattern:

1. Request enters the application
2. Global middleware executes in order of registration
3. Group middleware executes (if applicable)
4. Route-specific middleware executes
5. Route handler executes
6. Middleware executes in reverse order for the response
7. Response leaves the application

## Debugging Middleware

To debug middleware:

1. Use logging to track execution:
```rust
println!("Middleware: Processing request to {}", request.uri());
```

2. Check middleware order:
```rust
// Explicit ordering
router
    .middleware(first_middleware)
    .middleware(second_middleware)
    .middleware(third_middleware)
```

3. Test middleware in isolation:
```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_middleware() {
        let middleware = MyMiddleware::new();
        let request = Request::builder()
            .uri("/test")
            .body(Body::empty())
            .unwrap();
        let response = middleware
            .handle(request, next)
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
``` 