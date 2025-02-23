use axum::http::StatusCode;
use serde_json::Value;
use crate::framework::testing::TestCase;
use axum::body::{Body, to_bytes};
use std::str;
use bytes::Buf;
use axum::response::Response;

/// Helper function to read the entire body into bytes
async fn read_body(body: Body) -> Vec<u8> {
    let bytes = to_bytes(body, usize::MAX).await.unwrap_or_default();
    bytes.to_vec()
}

/// Trait for making assertions about HTTP responses
pub trait HttpAssertions {
    /// Assert that the response has a specific header
    fn assert_header(&mut self, name: &str, value: &str) -> &mut Self;
    
    /// Assert that the response is a redirect
    fn assert_redirect(&mut self, location: &str) -> &mut Self;
    
    /// Assert that the response contains a specific string
    fn assert_see(&mut self, text: &str) -> &mut Self;
    
    /// Assert that the response does not contain a specific string
    fn assert_dont_see(&mut self, text: &str) -> &mut Self;
    
    /// Assert that the response is a specific view
    fn assert_view(&mut self, name: &str) -> &mut Self;
    
    /// Assert that the response has validation errors
    fn assert_has_errors(&mut self, fields: &[&str]) -> &mut Self;
    
    /// Assert that the response has no validation errors
    fn assert_has_no_errors(&mut self) -> &mut Self;
}

impl HttpAssertions for TestCase {
    fn assert_header(&mut self, name: &str, value: &str) -> &mut Self {
        let response = self.response.as_ref().unwrap();
        let header_value = response.headers().get(name)
            .expect(&format!("Response is missing header: {}", name))
            .to_str()
            .unwrap();
            
        assert_eq!(
            header_value,
            value,
            "Expected header '{}' to be '{}' but got '{}'",
            name,
            value,
            header_value
        );
        
        self
    }
    
    fn assert_redirect(&mut self, location: &str) -> &mut Self {
        let response = self.response.as_ref().unwrap();
        
        assert!(
            response.status().is_redirection(),
            "Response status code {} is not a redirect",
            response.status()
        );
        
        self.assert_header("location", location)
    }
    
    fn assert_see(&mut self, text: &str) -> &mut Self {
        let bytes = self.get_body_bytes();
        let body = str::from_utf8(&bytes).unwrap();
        
        assert!(
            body.contains(text),
            "Unable to find '{}' in response body",
            text
        );
        
        self
    }
    
    fn assert_dont_see(&mut self, text: &str) -> &mut Self {
        let bytes = self.get_body_bytes();
        let body = str::from_utf8(&bytes).unwrap();
        
        assert!(
            !body.contains(text),
            "Found unexpected '{}' in response body",
            text
        );
        
        self
    }
    
    fn assert_view(&mut self, name: &str) -> &mut Self {
        // This would need to be implemented based on your view system
        // For now just check if the response contains the view name
        self.assert_see(name)
    }
    
    fn assert_has_errors(&mut self, fields: &[&str]) -> &mut Self {
        let response = self.response.as_ref().unwrap();
        
        // Expect validation errors to return 422 status
        assert_eq!(
            response.status(),
            StatusCode::UNPROCESSABLE_ENTITY,
            "Expected validation errors but got status code {}",
            response.status()
        );
        
        let bytes = self.get_body_bytes();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        let errors = body.get("errors")
            .expect("Response is missing 'errors' object");
            
        for field in fields {
            assert!(
                errors.get(field).is_some(),
                "Expected validation error for field '{}'",
                field
            );
        }
        
        self
    }
    
    fn assert_has_no_errors(&mut self) -> &mut Self {
        let bytes = self.get_body_bytes();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        
        assert!(
            !body.get("errors").is_some(),
            "Found unexpected validation errors in response"
        );
        
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router,
        routing::get,
        response::IntoResponse,
        http::{header, Response, StatusCode},
    };
    use serde_json::json;
    
    #[tokio::test]
    async fn test_http_assertions() {
        // Test redirect
        let app = Router::new().route("/redirect", get(|| async {
            Response::builder()
                .status(StatusCode::FOUND)
                .header(header::LOCATION, "/destination")
                .body(axum::body::Body::empty())
                .unwrap()
        }));
        
        let mut test_case = TestCase::new(app);
        test_case
            .get("/redirect")
            .await
            .assert_redirect("/destination");
            
        // Test validation errors
        let app = Router::new().route("/validate", get(|| async {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                axum::Json(json!({
                    "errors": {
                        "email": ["The email field is required"],
                        "password": ["The password field is required"]
                    }
                }))
            ).into_response()
        }));
        
        let mut test_case = TestCase::new(app);
        test_case
            .get("/validate")
            .await
            .assert_has_errors(&["email", "password"]);
    }
} 