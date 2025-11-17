pub mod api;
pub mod sdk;
pub mod service;

pub mod plugin {
    tonic::include_proto!("plugin");
}

pub use plugin_sdk_macros::*;
