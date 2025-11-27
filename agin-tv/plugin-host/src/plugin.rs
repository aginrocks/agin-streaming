use ginepro::LoadBalancedChannel;
use plugin_proto::PluginInfo;

pub struct Plugin {
    pub hostname: String,
    pub port: u16,
    pub status: PluginStatus,
    pub channel: LoadBalancedChannel,
}

pub enum PluginStatus {
    // Plugin has not responded to `GetInfo` yet
    Pending,
    // Plugin has responded, but could be unhealthy
    Discovered(PluginInfo),
}

impl Plugin {
    pub async fn new_connect(hostname: String, port: u16) -> Result<Self, anyhow::Error> {
        let channel = LoadBalancedChannel::builder((hostname.clone(), port))
            .channel()
            .await?;

        let plugin = Self {
            hostname,
            port,
            status: PluginStatus::Pending,
            channel,
        };
        Ok(plugin)
    }
}
