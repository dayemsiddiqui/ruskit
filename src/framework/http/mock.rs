use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use reqwest::header;
use serde::de::DeserializeOwned;
use crate::framework::http::error::HttpError;

pub struct MockHttp {
    responses: Arc<RwLock<HashMap<String, Vec<MockResponse>>>>,
}

#[derive(Clone)]
pub struct MockResponse {
    pub status: u16,
    pub body: String,
    pub headers: header::HeaderMap,
}

impl MockHttp {
    pub fn new() -> Self {
        Self {
            responses: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn mock_response(&self, path: &str, response: MockResponse) {
        let mut responses = self.responses.write().await;
        responses
            .entry(path.to_string())
            .or_insert_with(Vec::new)
            .push(response);
    }

    pub async fn mock_json<T: serde::Serialize>(&self, path: &str, status: u16, data: &T) -> Result<(), HttpError> {
        let response = MockResponse {
            status,
            body: serde_json::to_string(data)?,
            headers: {
                let mut headers = header::HeaderMap::new();
                headers.insert(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_static("application/json"),
                );
                headers
            },
        };
        self.mock_response(path, response).await;
        Ok(())
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, HttpError> {
        let responses = self.responses.read().await;
        let mock_responses = responses
            .get(path)
            .ok_or(HttpError::NoMockResponse)?;

        if mock_responses.is_empty() {
            return Err(HttpError::NoMockResponse);
        }

        let response = &mock_responses[0];
        Ok(serde_json::from_str(&response.body)?)
    }

    pub async fn post<T: DeserializeOwned>(&self, path: &str, _body: &impl serde::Serialize) -> Result<T, HttpError> {
        self.get(path).await
    }

    pub async fn put<T: DeserializeOwned>(&self, path: &str, _body: &impl serde::Serialize) -> Result<T, HttpError> {
        self.get(path).await
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, HttpError> {
        self.get(path).await
    }
} 