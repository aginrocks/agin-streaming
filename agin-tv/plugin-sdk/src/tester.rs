use std::sync::Arc;

use crate::{cache::CacheStub, context::Context};

pub fn new_context<S: Send + Sync + Clone + 'static>(state: S) -> Context<S> {
    Context {
        state: Arc::new(state),
        cache: Arc::new(CacheStub {}),
    }
}
