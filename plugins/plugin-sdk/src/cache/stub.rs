use std::time::Duration;

use async_trait::async_trait;

use crate::cache::{CacheError, CacheProvider};

#[derive(Clone, Default)]
pub struct CacheStub {}

impl CacheStub {
    pub fn new() -> Self {
        CacheStub {}
    }
}

#[async_trait]
impl CacheProvider for CacheStub {
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
