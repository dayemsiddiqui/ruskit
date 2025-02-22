pub mod web;
pub mod framework;
pub mod app;
pub mod routes;

// Re-export framework functionality
pub use framework::{
    // Core functionality
    bootstrap,
    middleware_stack,
    middleware_group,
    
    // Types and traits
    Middleware,
    RouteMiddlewareExt,
    RouterMiddlewareExt,
    WithMiddleware,
    presets,
    
    // Commonly used items
    prelude::*,
}; 