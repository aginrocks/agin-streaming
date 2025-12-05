use std::time::Duration;

use async_trait::async_trait;

use crate::cache::{CacheError, CacheProvider};

#[derive(Clone, Default)]
pub struct CacheManager {}

impl CacheManager {
    pub fn new() -> Self {
        CacheManager {}
    }
}

#[async_trait]
impl CacheProvider for CacheManager {
    async fn get(&self, _key: &str) -> Result<Option<String>, CacheError> {
        Ok(None)
    }

    async fn set(
        &self,
        _key: &str,
        _value: &str,
        _ttl: Option<Duration>,
    ) -> Result<(), CacheError> {
        Ok(())
    }
}
