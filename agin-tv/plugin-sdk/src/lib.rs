pub mod cache;
pub mod context;
pub mod handler;
pub mod plugin;
pub mod sdk;
pub(crate) mod services;

pub use context::_GetGenerics;
pub mod macros {
    pub use plugin_sdk_macros::*;
}

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;
