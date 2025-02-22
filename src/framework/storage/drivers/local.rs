use std::path::{Path, PathBuf};
use async_trait::async_trait;
use std::io::Result as IoResult;
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::framework::storage::StorageDriver;

#[derive(Debug)]
pub struct LocalDriver {
    root: PathBuf,
    url: String,
}

impl LocalDriver {
    pub fn new<P: AsRef<Path>>(root: P, url: &str) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
            url: url.to_string(),
        }
    }

    fn ensure_path(&self, path: &str) -> PathBuf {
        self.root.join(path)
    }
}

#[async_trait]
impl StorageDriver for LocalDriver {
    async fn get(&self, path: &str) -> IoResult<Vec<u8>> {
        let path = self.ensure_path(path);
        let mut file = File::open(path).await?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await?;
        Ok(contents)
    }

    async fn put(&self, path: &str, contents: &[u8]) -> IoResult<()> {
        let path = self.ensure_path(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let mut file = File::create(path).await?;
        file.write_all(contents).await
    }

    async fn delete(&self, path: &str) -> IoResult<()> {
        let path = self.ensure_path(path);
        fs::remove_file(path).await
    }

    async fn exists(&self, path: &str) -> bool {
        let path = self.ensure_path(path);
        path.exists()
    }

    async fn size(&self, path: &str) -> IoResult<u64> {
        let path = self.ensure_path(path);
        let metadata = fs::metadata(path).await?;
        Ok(metadata.len())
    }

    async fn copy(&self, from: &str, to: &str) -> IoResult<()> {
        let from_path = self.ensure_path(from);
        let to_path = self.ensure_path(to);
        if let Some(parent) = to_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::copy(from_path, to_path).await?;
        Ok(())
    }

    async fn move_file(&self, from: &str, to: &str) -> IoResult<()> {
        let from_path = self.ensure_path(from);
        let to_path = self.ensure_path(to);
        if let Some(parent) = to_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::rename(from_path, to_path).await
    }

    async fn url(&self, path: &str) -> String {
        format!("{}/{}", self.url.trim_end_matches('/'), path.trim_start_matches('/'))
    }

    async fn make_directory(&self, path: &str) -> IoResult<()> {
        let path = self.ensure_path(path);
        fs::create_dir_all(path).await
    }

    async fn delete_directory(&self, path: &str) -> IoResult<()> {
        let path = self.ensure_path(path);
        fs::remove_dir_all(path).await
    }
} 