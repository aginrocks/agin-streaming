use color_eyre::Result;
use dashmap::{DashMap, DashSet};
use plugin_proto::ServiceType;
use tracing::warn;

use crate::plugin::Plugin;

#[derive(Default)]
pub struct PluginHost {
    plugins: DashMap<(String, u16), Plugin>,
    providers: DashMap<(ServiceType, String), DashSet<(String, u16)>>,
}

impl PluginHost {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn register_plugin(&self, plugin: Plugin) {
        self.plugins
            .insert((plugin.hostname.clone(), plugin.port), plugin.clone());

        for service in plugin.info.services {
            for provider in service.providers {
                if let Err(e) = self.set_provider(
                    service.r#type,
                    provider,
                    plugin.hostname.clone(),
                    plugin.port,
                ) {
                    warn!("Failed to set provider: {}", e);
                }
            }
        }
    }

    fn set_provider(
        &self,
        service_type: i32,
        provider: String,
        hostname: String,
        port: u16,
    ) -> Result<()> {
        self.providers
            .entry((service_type.try_into()?, provider))
            .or_default()
            .insert((hostname, port));

        Ok(())
    }
}
