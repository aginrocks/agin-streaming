use std::sync::Arc;

use crate::cache::{CacheProvider, CacheStub};

#[derive(Clone)]
pub struct Context<T: Send + Sync + 'static> {
    /// Shared state across the entire plugin lifecycle
    pub state: Arc<T>,

    /// Cache manager for the plugin
    pub cache: Arc<dyn CacheProvider>,
}

impl<T: Send + Sync + Clone + 'static> Context<T> {
    pub fn new_stub(state: T) -> Context<T> {
        Context {
            state: Arc::new(state.clone()),
            cache: Arc::new(CacheStub {}),
        }
    }
}

// needed for proc macro
#[doc(hidden)]
pub trait _GetGenerics {
    type T;
}
impl<T: Send + Sync + 'static> _GetGenerics for Context<T> {
    type T = T;
}
