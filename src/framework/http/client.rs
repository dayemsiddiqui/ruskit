use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;
use reqwest::{Client as ReqwestClient, Response};
use serde::{de::DeserializeOwned, Serialize};
use crate::framework::http::{
    config::{EndpointConfig, LoadBalancerConfig, RetryConfig},
    error::HttpError,
    queue::RequestQueue,
};

pub struct Http {
    client: ReqwestClient,
    load_balancer: Arc<LoadBalancerConfig>,
    retry_config: Option<RetryConfig>,
    request_queue: Option<Arc<RequestQueue>>,
}

impl Http {
    pub fn new(endpoints: Vec<EndpointConfig>) -> Self {
        if endpoints.is_empty() {
            panic!("At least one endpoint must be provided");
        }

        Self {
            client: ReqwestClient::new(),
            load_balancer: Arc::new(LoadBalancerConfig::new(endpoints)),
            retry_config: None,
            request_queue: None,
        }
    }

    pub fn with_retry(mut self, config: RetryConfig) -> Self {
        self.retry_config = Some(config);
        self
    }

    pub fn with_queue(mut self, max_concurrent: usize) -> Self {
        self.request_queue = Some(Arc::new(RequestQueue::new(max_concurrent)));
        self
    }

    fn get_next_endpoint(&self) -> Result<EndpointConfig, HttpError> {
        let current = self.load_balancer.current_index.fetch_add(1, Ordering::SeqCst);
        let index = current % self.load_balancer.endpoints.len();
        Ok(self.load_balancer.endpoints[index].clone())
    }

    async fn execute_with_retry<F, Fut, T>(&self, f: F) -> Result<T, HttpError>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<T, HttpError>> + Send,
        T: Send + 'static,
    {
        let retry_config = match &self.retry_config {
            Some(config) => config,
            None => return f().await,
        };

        let mut delay = retry_config.initial_delay;
        let mut attempts = 0;

        loop {
            match f().await {
                Ok(result) => return Ok(result),
                Err(e) if attempts < retry_config.max_retries => {
                    attempts += 1;
                    tokio::time::sleep(delay).await;
                    delay = std::cmp::min(
                        Duration::from_secs_f64(delay.as_secs_f64() * retry_config.multiplier),
                        retry_config.max_delay,
                    );
                }
                Err(e) => return Err(e),
            }
        }
    }

    async fn execute_request<F, Fut>(&self, f: F) -> Result<reqwest::Response, HttpError>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<reqwest::Response, HttpError>> + Send + 'static,
    {
        match &self.request_queue {
            Some(queue) => queue.enqueue(f).await,
            None => f().await,
        }
    }

    pub async fn get<T>(&self, path: &str) -> Result<T, HttpError>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let client = self.client.clone();
        let endpoint = self.get_next_endpoint()?;
        let path = path.to_string();

        let result = self.execute_request(move || async move {
            let url = format!("{}{}", endpoint.url, path);
            let mut request = client.get(&url);

            if let Some(token) = endpoint.api_token {
                request = request.header("Authorization", format!("Bearer {}", token));
            }

            for (key, value) in endpoint.headers {
                request = request.header(key.unwrap(), value);
            }

            request.send().await.map_err(HttpError::from)
        }).await?;

        result.json().await.map_err(HttpError::from)
    }

    pub async fn post<T>(&self, path: &str, json: &impl Serialize) -> Result<T, HttpError>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let client = self.client.clone();
        let endpoint = self.get_next_endpoint()?;
        let path = path.to_string();
        let json = serde_json::to_value(json)?;

        let result = self.execute_request(move || async move {
            let url = format!("{}{}", endpoint.url, path);
            let mut request = client.post(&url).json(&json);

            if let Some(token) = endpoint.api_token {
                request = request.header("Authorization", format!("Bearer {}", token));
            }

            for (key, value) in endpoint.headers {
                request = request.header(key.unwrap(), value);
            }

            request.send().await.map_err(HttpError::from)
        }).await?;

        result.json().await.map_err(HttpError::from)
    }

    pub async fn put<T>(&self, path: &str, json: &impl Serialize) -> Result<T, HttpError>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let client = self.client.clone();
        let endpoint = self.get_next_endpoint()?;
        let path = path.to_string();
        let json = serde_json::to_value(json)?;

        let result = self.execute_request(move || async move {
            let url = format!("{}{}", endpoint.url, path);
            let mut request = client.put(&url).json(&json);

            if let Some(token) = endpoint.api_token {
                request = request.header("Authorization", format!("Bearer {}", token));
            }

            for (key, value) in endpoint.headers {
                request = request.header(key.unwrap(), value);
            }

            request.send().await.map_err(HttpError::from)
        }).await?;

        result.json().await.map_err(HttpError::from)
    }

    pub async fn delete<T>(&self, path: &str) -> Result<T, HttpError>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let client = self.client.clone();
        let endpoint = self.get_next_endpoint()?;
        let path = path.to_string();

        let result = self.execute_request(move || async move {
            let url = format!("{}{}", endpoint.url, path);
            let mut request = client.delete(&url);

            if let Some(token) = endpoint.api_token {
                request = request.header("Authorization", format!("Bearer {}", token));
            }

            for (key, value) in endpoint.headers {
                request = request.header(key.unwrap(), value);
            }

            request.send().await.map_err(HttpError::from)
        }).await?;

        result.json().await.map_err(HttpError::from)
    }
} 