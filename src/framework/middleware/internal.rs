use axum::{
    middleware::Next,
    response::Response,
    http::Request,
    body::Body,
    routing::MethodRouter,
    Router,
    middleware::from_fn,
};

/// Route builder with middleware support
pub(crate) struct RouteBuilder {
    route: MethodRouter,
    middlewares: Vec<Middleware>,
}

impl RouteBuilder {
    pub fn new(route: MethodRouter) -> Self {
        Self {
            route,
            middlewares: Vec::new(),
        }
    }

    pub fn middleware(mut self, middleware: Middleware) -> Self {
        self.middlewares.push(middleware);
        self
    }

    pub fn middleware_vec(mut self, middlewares: Vec<Middleware>) -> Self {
        self.middlewares.extend(middlewares);
        self
    }

    pub fn build(self) -> MethodRouter {
        let mut route = self.route;
        for middleware in self.middlewares.into_iter().rev() {
            route = route.layer(from_fn(move |req: Request<Body>, next| {
                let middleware = middleware.clone();
                async move {
                    middleware.handle(req, next).await.unwrap_or_else(|resp| resp)
                }
            }));
        }
        route
    }
}

/// Middleware configuration for the application
#[derive(Default, Clone)]
pub struct MiddlewareStack {
    pub(crate) global: Vec<Middleware>,
    pub(crate) groups: Vec<(String, Vec<Middleware>)>,
}

impl MiddlewareStack {
    pub fn new() -> Self {
        Self {
            global: Vec::new(),
            groups: Vec::new(),
        }
    }

    pub fn add(&mut self, middleware: Middleware) {
        self.global.push(middleware);
    }

    pub fn group(&mut self, name: &str, middlewares: Vec<Middleware>) {
        self.groups.push((name.to_string(), middlewares));
    }

    pub fn global(&self) -> Vec<Middleware> {
        self.global.clone()
    }

    pub fn group_middlewares(&self, name: &str) -> Option<Vec<Middleware>> {
        self.groups
            .iter()
            .find(|(group_name, _)| group_name == name)
            .map(|(_, middlewares)| middlewares.clone())
    }
}

// Extension trait for MethodRouter to add middleware methods
pub trait RouteMiddlewareExt {
    fn with_middleware(self, middleware: Middleware) -> RouteBuilder;
    fn with_middlewares(self, middlewares: Vec<Middleware>) -> RouteBuilder;
}

impl RouteMiddlewareExt for MethodRouter {
    fn with_middleware(self, middleware: Middleware) -> RouteBuilder {
        RouteBuilder::new(self).middleware(middleware)
    }

    fn with_middlewares(self, middlewares: Vec<Middleware>) -> RouteBuilder {
        RouteBuilder::new(self).middleware_vec(middlewares)
    }
}

// Extension trait for Router to add middleware methods
pub trait RouterMiddlewareExt {
    fn with_middleware(self, middleware: Middleware) -> Router;
    fn with_middlewares(self, middlewares: Vec<Middleware>) -> Router;
}

impl RouterMiddlewareExt for Router {
    fn with_middleware(self, middleware: Middleware) -> Router {
        self.layer(from_fn(move |req: Request<Body>, next| {
            let middleware = middleware.clone();
            async move {
                middleware.handle(req, next).await.unwrap_or_else(|resp| resp)
            }
        }))
    }

    fn with_middlewares(self, middlewares: Vec<Middleware>) -> Router {
        let mut router = self;
        for middleware in middlewares {
            router = router.with_middleware(middleware);
        }
        router
    }
}

/// Enum representing all available middleware
#[derive(Clone)]
pub enum Middleware {
    Cors(super::presets::Cors),
    TrimStrings(super::presets::TrimStrings),
}

impl Middleware {
    pub async fn handle(
        &self,
        request: Request<Body>,
        next: Next,
    ) -> Result<Response, Response> {
        match self {
            Middleware::Cors(cors) => cors.handle(request, next).await,
            Middleware::TrimStrings(trim) => trim.handle(request, next).await,
        }
    }
} 