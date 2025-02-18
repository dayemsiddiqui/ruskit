# Ruskit Documentation

Welcome to the Ruskit documentation! Ruskit is a web application framework with expressive, elegant syntax inspired by Laravel, built for Rust. We believe development must be an enjoyable and creative experience. Ruskit takes the pain out of web development by easing common tasks used in many web projects.

## Table of Contents

1. [Getting Started](./getting-started.md) (Coming Soon)
2. [Routing](./routing.md)
3. [Controllers](./controllers.md) (Coming Soon)
4. [Requests & Responses](./requests-responses.md) (Coming Soon)
5. [Views](./views.md) (Coming Soon)
6. [Database](./database.md) (Coming Soon)
7. [Authentication](./authentication.md) (Coming Soon)
8. [Authorization](./authorization.md) (Coming Soon)
9. [Validation](./validation.md) (Coming Soon)
10. [Error Handling](./error-handling.md) (Coming Soon)
11. [Testing](./testing.md) (Coming Soon)

## Quick Start

### Installation

Add Ruskit to your project:

```toml
[dependencies]
ruskit = "0.1.0"  # Coming Soon
```

### Basic Usage

Create a new route in your `web.rs`:

```rust
use axum::{
    Router,
    routing::get,
    response::Json,
};
use serde_json::{json, Value};

// Define a route handler
async fn hello() -> Json<Value> {
    Json(json!({ "message": "Hello, Ruskit!" }))
}

// Register your routes
pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(hello))
}
```

### Run Your Application

```rust
#[tokio::main]
async fn main() {
    let app = web::routes();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    println!("Server running on http://{}", addr);
    
    axum::serve(
        TcpListener::bind(addr).await.unwrap(),
        app
    ).await.unwrap();
}
```

## Features

- 🚀 **Fast & Reliable**: Built on top of Axum and Tokio
- 🔒 **Secure**: Security-first approach to web development
- 📦 **Modular**: Use what you need, leave what you don't
- 🛠️ **Developer Friendly**: Intuitive APIs and excellent documentation
- ⚡ **High Performance**: Leveraging Rust's performance capabilities
- 🧪 **Testing**: First-class testing support

## Contributing

We welcome contributions! Please see our [Contributing Guide](./contributing.md) (Coming Soon) for details.

## License

The Ruskit framework is open-sourced software licensed under the [MIT license](../LICENSE). 