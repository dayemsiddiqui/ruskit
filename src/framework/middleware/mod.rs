use axum::{Router, routing::MethodRouter};

mod internal;
pub mod presets;

// Re-export the middleware types that users need
pub use internal::{Middleware, MiddlewareStack, RouteMiddlewareExt, RouterMiddlewareExt};
pub use presets::{Cors, TrimStrings};

/// Extension methods for applying middleware to routes and routers
pub trait WithMiddleware {
    /// Apply a single middleware
    fn middleware(self, middleware: impl Into<Middleware>) -> Self;
    
    /// Apply multiple middleware
    fn middlewares<I>(self, middlewares: Vec<I>) -> Self 
    where
        I: Into<Middleware>;
}

impl WithMiddleware for MethodRouter {
    fn middleware(self, middleware: impl Into<Middleware>) -> Self {
        internal::RouteMiddlewareExt::with_middleware(self, middleware.into()).build()
    }

    fn middlewares<I>(self, middlewares: Vec<I>) -> Self 
    where
        I: Into<Middleware>
    {
        internal::RouteMiddlewareExt::with_middlewares(
            self,
            middlewares.into_iter().map(Into::into).collect()
        ).build()
    }
}

impl WithMiddleware for Router {
    fn middleware(self, middleware: impl Into<Middleware>) -> Self {
        internal::RouterMiddlewareExt::with_middleware(self, middleware.into())
    }

    fn middlewares<I>(self, middlewares: Vec<I>) -> Self 
    where
        I: Into<Middleware>
    {
        internal::RouterMiddlewareExt::with_middlewares(
            self,
            middlewares.into_iter().map(Into::into).collect()
        )
    }
} 