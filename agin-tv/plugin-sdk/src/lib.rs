pub mod api;
pub mod context;
pub mod sdk;
pub mod service;

pub mod plugin {
    tonic::include_proto!("plugin");
}

pub use plugin_sdk_macros::*;

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;
