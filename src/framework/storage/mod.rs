use std::path::Path;
use async_trait::async_trait;
use std::io::{Error as IoError, Result as IoResult};
use tokio::fs::File;
use std::sync::Arc;
use tokio::sync::RwLock;
use once_cell::sync::OnceCell;
use std::fmt::Debug;

pub mod drivers;
pub mod config;

static STORAGE_DRIVER: OnceCell<Arc<RwLock<Box<dyn StorageDriver + Send + Sync>>>> = OnceCell::new();

#[async_trait]
pub trait StorageDriver: Debug {
    /// Get the contents of a file
    async fn get(&self, path: &str) -> IoResult<Vec<u8>>;
    
    /// Write the contents of a file
    async fn put(&self, path: &str, contents: &[u8]) -> IoResult<()>;
    
    /// Delete the file at a given path
    async fn delete(&self, path: &str) -> IoResult<()>;
    
    /// Determine if a file exists
    async fn exists(&self, path: &str) -> bool;
    
    /// Get the size of a file in bytes
    async fn size(&self, path: &str) -> IoResult<u64>;
    
    /// Copy a file to a new location
    async fn copy(&self, from: &str, to: &str) -> IoResult<()>;
    
    /// Move a file to a new location
    async fn move_file(&self, from: &str, to: &str) -> IoResult<()>;
    
    /// Get a URL for the file at the given path
    async fn url(&self, path: &str) -> String;
    
    /// Create a directory at the given path
    async fn make_directory(&self, path: &str) -> IoResult<()>;
    
    /// Delete a directory at the given path
    async fn delete_directory(&self, path: &str) -> IoResult<()>;
}

/// A Laravel-like Storage facade for easy file operations
pub struct Storage;

impl Storage {
    /// Get the underlying storage driver
    pub fn driver() -> Arc<RwLock<Box<dyn StorageDriver + Send + Sync>>> {
        Arc::clone(STORAGE_DRIVER.get().expect("Storage driver not initialized"))
    }

    /// Get the contents of a file
    pub async fn get(path: &str) -> IoResult<Vec<u8>> {
        let driver = Self::driver();
        let driver = driver.read().await;
        driver.get(path).await
    }

    /// Write the contents of a file
    pub async fn put(path: &str, contents: &[u8]) -> IoResult<()> {
        let driver = Self::driver();
        let driver = driver.read().await;
        driver.put(path, contents).await
    }

    /// Delete the file at a given path
    pub async fn delete(path: &str) -> IoResult<()> {
        let driver = Self::driver();
        let driver = driver.read().await;
        driver.delete(path).await
    }

    /// Determine if a file exists
    pub async fn exists(path: &str) -> bool {
        let driver = Self::driver();
        let driver = driver.read().await;
        driver.exists(path).await
    }

    /// Get the size of a file in bytes
    pub async fn size(path: &str) -> IoResult<u64> {
        let driver = Self::driver();
        let driver = driver.read().await;
        driver.size(path).await
    }

    /// Copy a file to a new location
    pub async fn copy(from: &str, to: &str) -> IoResult<()> {
        let driver = Self::driver();
        let driver = driver.read().await;
        driver.copy(from, to).await
    }

    /// Move a file to a new location
    pub async fn move_file(from: &str, to: &str) -> IoResult<()> {
        let driver = Self::driver();
        let driver = driver.read().await;
        driver.move_file(from, to).await
    }

    /// Get a URL for the file at the given path
    pub async fn url(path: &str) -> String {
        let driver = Self::driver();
        let driver = driver.read().await;
        driver.url(path).await
    }

    /// Create a directory at the given path
    pub async fn make_directory(path: &str) -> IoResult<()> {
        let driver = Self::driver();
        let driver = driver.read().await;
        driver.make_directory(path).await
    }

    /// Delete a directory at the given path
    pub async fn delete_directory(path: &str) -> IoResult<()> {
        let driver = Self::driver();
        let driver = driver.read().await;
        driver.delete_directory(path).await
    }
} 