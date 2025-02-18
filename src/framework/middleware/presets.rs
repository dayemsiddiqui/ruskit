use axum::{
    middleware::Next,
    response::Response,
    http::Request,
    body::Body,
    http::header,
};

/// CORS middleware
#[derive(Clone)]
pub struct Cors {
    allow_origin: String,
    allow_methods: String,
    allow_headers: String,
}

impl Cors {
    pub fn new(allow_origin: &str) -> Self {
        Self {
            allow_origin: allow_origin.to_string(),
            allow_methods: "GET, POST, PUT, DELETE, OPTIONS".to_string(),
            allow_headers: "Content-Type, Authorization".to_string(),
        }
    }

    pub fn with_methods(mut self, methods: &str) -> Self {
        self.allow_methods = methods.to_string();
        self
    }

    pub fn with_headers(mut self, headers: &str) -> Self {
        self.allow_headers = headers.to_string();
        self
    }

    pub(crate) async fn handle(
        &self,
        request: Request<Body>,
        next: Next,
    ) -> Result<Response, Response> {
        let mut response = next.run(request).await;
        
        response.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            self.allow_origin.parse().unwrap(),
        );
        response.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            self.allow_methods.parse().unwrap(),
        );
        response.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            self.allow_headers.parse().unwrap(),
        );

        Ok(response)
    }
}

/// Trim strings middleware
#[derive(Clone)]
pub struct TrimStrings;

impl TrimStrings {
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn handle(
        &self,
        request: Request<Body>,
        next: Next,
    ) -> Result<Response, Response> {
        // TODO: Implement string trimming for request body
        Ok(next.run(request).await)
    }
}

// Implement From traits for easy conversion
impl From<Cors> for super::internal::Middleware {
    fn from(cors: Cors) -> Self {
        Self::Cors(cors)
    }
}

impl From<TrimStrings> for super::internal::Middleware {
    fn from(trim: TrimStrings) -> Self {
        Self::TrimStrings(trim)
    }
} 