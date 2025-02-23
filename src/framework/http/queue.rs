use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::Mutex;
use crate::framework::http::error::HttpError;

pub struct RequestQueue {
    queue: Mutex<VecDeque<PendingRequest>>,
    max_concurrent: usize,
    current_concurrent: AtomicUsize,
}

type RequestFuture = Pin<Box<dyn Future<Output = Result<reqwest::Response, HttpError>> + Send>>;

pub struct PendingRequest {
    request: Box<dyn FnOnce() -> RequestFuture + Send>,
}

impl RequestQueue {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            max_concurrent,
            current_concurrent: AtomicUsize::new(0),
        }
    }

    pub async fn enqueue<F, Fut>(&self, f: F) -> Result<reqwest::Response, HttpError>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Result<reqwest::Response, HttpError>> + Send + 'static,
    {
        let current = self.current_concurrent.load(Ordering::SeqCst);
        
        if current < self.max_concurrent {
            self.current_concurrent.fetch_add(1, Ordering::SeqCst);
            let result = f().await;
            self.current_concurrent.fetch_sub(1, Ordering::SeqCst);
            result
        } else {
            let request = PendingRequest {
                request: Box::new(move || Box::pin(f())),
            };
            
            let mut queue = self.queue.lock().await;
            queue.push_back(request);
            
            if queue.len() > 1000 { // Arbitrary limit to prevent memory issues
                return Err(HttpError::QueueFull);
            }
            
            self.process_queue().await
        }
    }

    async fn process_queue(&self) -> Result<reqwest::Response, HttpError> {
        loop {
            let current = self.current_concurrent.load(Ordering::SeqCst);
            if current < self.max_concurrent {
                let mut queue = self.queue.lock().await;
                if let Some(request) = queue.pop_front() {
                    self.current_concurrent.fetch_add(1, Ordering::SeqCst);
                    let result = (request.request)().await;
                    self.current_concurrent.fetch_sub(1, Ordering::SeqCst);
                    return result;
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }
} 