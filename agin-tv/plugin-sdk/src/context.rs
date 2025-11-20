use std::sync::Arc;

use crate::api::cache::CacheManager;

pub struct Context<T: Send + Sync + 'static> {
    /// Shared state across the entire plugin lifecycle
    pub state: Arc<T>,

    /// Cache manager for the plugin
    pub cache: Arc<CacheManager>,
}
