pub mod manager;
pub mod stub;

pub use manager::CacheManager;
pub use stub::CacheStub;

use std::time::Duration;

use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {}

#[async_trait]
pub trait CacheProvider: Send + Sync + 'static {
    async fn get(&self, key: &str) -> Result<Option<String>, CacheError>;
    async fn set(&self, key: &str, value: &str, ttl: Option<Duration>) -> Result<(), CacheError>;
}
