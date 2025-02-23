use sea_orm::DatabaseConnection;
use axum::response::Response;
use axum::http::StatusCode;
use serde_json::Value;
use std::future::Future;
use axum::body::{Body, to_bytes};
use axum::http::Request;
use axum::Router;
use tower::ServiceExt;
use bytes::Buf;
use axum::body::Bytes;
use std::collections::HashMap;

pub mod assertions;
pub mod database;
pub mod http;

/// Helper function to read the entire body into bytes
pub async fn read_body(body: Body) -> Vec<u8> {
    let bytes = to_bytes(body, usize::MAX).await.unwrap_or_default();
    bytes.to_vec()
}

/// Main test case struct that provides a fluent API for testing
pub struct TestCase {
    app: Router,
    response: Option<Response<Body>>,
    response_bytes: Option<Vec<u8>>,
    db: DatabaseConnection,
    auth_header: Option<String>,
}

impl TestCase {
    /// Create a new test case
    pub fn new(app: Router) -> Self {
        Self {
            app,
            response: None,
            response_bytes: None,
            db: DatabaseConnection::default(),
            auth_header: None,
        }
    }

    /// Act as a specific user for the test
    pub fn acting_as(&mut self, user_id: i64) -> &mut Self {
        self.auth_header = Some(format!("Bearer {}", user_id));
        self
    }

    /// Make a GET request
    pub async fn get(&mut self, uri: &str) -> &mut Self {
        self.send_request("GET", uri, None).await
    }

    /// Make a POST request with optional JSON body
    pub async fn post(&mut self, uri: &str, json: Option<Value>) -> &mut Self {
        self.send_request("POST", uri, json).await
    }

    /// Make a PUT request with optional JSON body
    pub async fn put(&mut self, uri: &str, json: Option<Value>) -> &mut Self {
        self.send_request("PUT", uri, json).await
    }

    /// Make a DELETE request
    pub async fn delete(&mut self, uri: &str) -> &mut Self {
        self.send_request("DELETE", uri, None).await
    }

    /// Assert response status code
    pub fn assert_status(&self, status: StatusCode) -> &Self {
        assert_eq!(
            self.response.as_ref().unwrap().status(),
            status,
            "Expected status code {} but got {}",
            status,
            self.response.as_ref().unwrap().status()
        );
        self
    }

    /// Assert response is OK (200)
    pub fn assert_ok(&self) -> &Self {
        self.assert_status(StatusCode::OK)
    }

    /// Assert response is Created (201)
    pub fn assert_created(&self) -> &Self {
        self.assert_status(StatusCode::CREATED)
    }

    /// Get the response body bytes, reading them if necessary
    fn get_body_bytes(&mut self) -> Vec<u8> {
        if let Some(bytes) = self.response_bytes.take() {
            return bytes;
        }

        let response = self.response.as_mut().unwrap();
        let bytes = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                let body = std::mem::replace(response.body_mut(), Body::empty());
                read_body(body).await
            });
        
        self.response_bytes = Some(bytes.clone());
        bytes
    }

    /// Assert response matches JSON
    pub fn assert_json(&mut self, expected: Value) -> &mut Self {
        let bytes = self.get_body_bytes();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        
        assert_eq!(
            body,
            expected,
            "Response JSON does not match expected value"
        );
        self
    }

    /// Assert JSON response contains specific key
    pub fn assert_json_has(&mut self, key: &str) -> &mut Self {
        let bytes = self.get_body_bytes();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        
        assert!(
            body.get(key).is_some(),
            "Response JSON missing key: {}",
            key
        );
        self
    }

    /// Internal helper to send HTTP requests
    async fn send_request(&mut self, method: &str, uri: &str, json: Option<Value>) -> &mut Self {
        let mut builder = Request::builder()
            .method(method)
            .uri(uri);

        // Add auth header if set
        if let Some(auth) = &self.auth_header {
            builder = builder.header("Authorization", auth);
        }

        let body = match json {
            Some(json) => Body::from(serde_json::to_vec(&json).unwrap()),
            None => Body::empty(),
        };

        let request = builder
            .body(body)
            .unwrap();

        let response = self.app
            .clone()
            .oneshot(request)
            .await
            .unwrap();

        self.response = Some(response);
        self
    }
}

/// Helper function to create a new test case
pub fn test(app: Router) -> TestCase {
    TestCase::new(app)
}

/// Helper trait for database assertions
pub trait DatabaseAssertions {
    fn assert_database_has(&self, table: &str, data: Value) -> &Self;
    fn assert_database_missing(&self, table: &str, data: Value) -> &Self;
    fn assert_database_count(&self, table: &str, count: i64) -> &Self;
}

/// Helper trait for running tests in a transaction
pub trait TransactionTest {
    fn run_in_transaction<F, Fut>(&self, test: F) -> Fut 
    where
        F: FnOnce() -> Fut + Clone,
        Fut: Future<Output = ()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::routing::get;
    use serde_json::json;

    async fn test_handler() -> &'static str {
        "Hello, World!"
    }

    #[tokio::test]
    async fn test_basic_request() {
        let app = Router::new().route("/", get(test_handler));
        
        test(app)
            .get("/")
            .await
            .assert_ok()
            .assert_status(StatusCode::OK);
    }

    #[tokio::test]
    async fn test_json_assertions() {
        let app = Router::new().route("/", get(|| async { 
            axum::Json(json!({"message": "success"}))
        }));
        
        test(app)
            .get("/")
            .await
            .assert_ok()
            .assert_json(json!({"message": "success"}))
            .assert_json_has("message");
    }
} 