pub mod cli;
pub mod web;
pub mod bootstrap;
pub mod framework;
pub mod app;

// Re-export framework types
pub use framework::middleware::{
    Middleware,
    RouteMiddlewareExt,
    RouterMiddlewareExt,
    WithMiddleware,
    presets,
};

// Re-export bootstrap functions
pub use bootstrap::app::{
    bootstrap,
    middleware_stack,
    middleware_group,
}; 