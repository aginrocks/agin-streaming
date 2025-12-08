use std::sync::Arc;

use plugin_proto::{PluginInfo, info_provider_service_server::InfoProviderService};
use tonic::{Request, Response, Status};
use tracing::info;

use crate::sdk::PluginSdk;

#[derive(Clone)]
pub struct InfoProvider<T: Send + Sync + Clone + 'static> {
    pub sdk: Arc<PluginSdk<T>>,
}

#[tonic::async_trait]
impl<T: Send + Sync + Clone + 'static> InfoProviderService for InfoProvider<T> {
    async fn get_plugin_info(&self, _: Request<()>) -> Result<Response<PluginInfo>, Status> {
        info!("Received request");
        let info = PluginInfo {
            id: env!("CARGO_PKG_NAME").to_string(),
            // TODO: Create a way of setting the display name
            display_name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            services: self.sdk.manifests.clone(),
        };

        Ok(Response::new(info))
    }
}
