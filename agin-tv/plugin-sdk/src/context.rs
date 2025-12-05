use std::sync::Arc;

use crate::cache::CacheProvider;

#[derive(Clone)]
pub struct Context<T: Send + Sync + 'static> {
    /// Shared state across the entire plugin lifecycle
    pub state: Arc<T>,

    /// Cache manager for the plugin
    pub cache: Arc<dyn CacheProvider>,
}

// needed for proc macro
#[doc(hidden)]
pub trait _GetGenerics {
    type T;
}
impl<T: Send + Sync + 'static> _GetGenerics for Context<T> {
    type T = T;
}
