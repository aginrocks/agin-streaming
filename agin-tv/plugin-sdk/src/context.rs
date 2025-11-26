use std::sync::Arc;

use crate::cache::CacheManager;

#[derive(Clone)]
pub struct Context<T: Send + Sync + 'static> {
    /// Shared state across the entire plugin lifecycle
    pub state: Arc<T>,

    /// Cache manager for the plugin
    pub cache: Arc<CacheManager>,
}
