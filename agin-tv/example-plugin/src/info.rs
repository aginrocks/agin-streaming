use plugin_sdk::service::AginService;
use tonic::{Request, Response, Status};

use crate::plugin::{
    PluginInfo,
    info_provider_server::{InfoProvider, InfoProviderServer},
};

#[derive(Debug, Default)]
pub struct Info {}

impl AginService for InfoProviderServer<Info> {
    fn metadata(&self) -> plugin_sdk::plugin::Service {
        todo!()
    }
}

#[tonic::async_trait]
impl InfoProvider for Info {
    async fn get_plugin_info(&self, request: Request<()>) -> Result<Response<PluginInfo>, Status> {
        println!("Got a request");

        let reply = PluginInfo {
            id: "1".into(),
            display_name: "a".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            services: vec![],
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
