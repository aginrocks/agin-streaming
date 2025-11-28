use std::sync::Arc;

use ginepro::LoadBalancedChannel;
use plugin_proto::PluginInfo;
use tokio::sync::RwLock;

use crate::clients::Clients;

#[derive(Clone)]
pub struct Plugin {
    pub hostname: String,
    pub port: u16,
    pub info: PluginInfo,
    pub clients: Arc<RwLock<Clients>>,
}

impl Plugin {
    pub async fn new_connect(hostname: String, port: u16) -> Result<Self, anyhow::Error> {
        let channel = LoadBalancedChannel::builder((hostname.clone(), port))
            .channel()
            .await?;

        let mut clients = Clients::from_channel(channel.clone());

        let info_request = tonic::Request::new(());
        let info = clients.info.get_plugin_info(info_request).await?;

        let plugin = Self {
            hostname,
            port,
            clients: Arc::new(RwLock::new(clients)),
            info: info.into_inner(),
        };
        Ok(plugin)
    }

    pub async fn get_info(&self) {}
}
