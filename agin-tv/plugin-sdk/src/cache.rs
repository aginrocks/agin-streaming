use std::time::Duration;

use thiserror::Error;

#[derive(Clone, Default)]
pub struct CacheManager {}

#[derive(Debug, Error)]
pub enum CacheError {}

impl CacheManager {
    pub fn new() -> Self {
        CacheManager {}
    }

    pub fn get(&self, key: impl Into<String>) -> Result<Option<String>, CacheError> {
        Ok(None)
    }

    pub fn set(
        &self,
        key: impl Into<String>,
        value: impl Into<String>,
        ttl: Option<Duration>,
    ) -> Result<(), CacheError> {
        Ok(())
    }
}
