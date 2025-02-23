use reqwest::header;
use std::time::Duration;
use std::sync::atomic::AtomicUsize;

#[derive(Clone)]
pub struct EndpointConfig {
    pub url: String,
    pub api_token: Option<String>,
    pub headers: header::HeaderMap,
}

impl EndpointConfig {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            api_token: None,
            headers: header::HeaderMap::new(),
        }
    }

    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.api_token = Some(token.into());
        self
    }

    pub fn with_header(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.headers.insert(
            header::HeaderName::from_bytes(key.as_ref().as_bytes()).unwrap(),
            header::HeaderValue::from_str(value.as_ref()).unwrap(),
        );
        self
    }
}

pub struct LoadBalancerConfig {
    pub(crate) endpoints: Vec<EndpointConfig>,
    pub(crate) current_index: AtomicUsize,
}

impl LoadBalancerConfig {
    pub fn new(endpoints: Vec<EndpointConfig>) -> Self {
        Self {
            endpoints,
            current_index: AtomicUsize::new(0),
        }
    }
}

#[derive(Clone)]
pub struct RetryConfig {
    pub max_retries: usize,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_retries(mut self, retries: usize) -> Self {
        self.max_retries = retries;
        self
    }

    pub fn with_initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    pub fn with_max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    pub fn with_multiplier(mut self, multiplier: f64) -> Self {
        self.multiplier = multiplier;
        self
    }
} 