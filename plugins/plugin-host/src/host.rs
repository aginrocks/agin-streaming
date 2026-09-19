pub mod search;

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

    pub fn has_plugin(&self, hostname: &str, port: u16) -> bool {
        self.plugins.contains_key(&(hostname.to_string(), port))
    }

    pub async fn register_plugin(&self, plugin: Plugin) {
        let old = self
            .plugins
            .insert((plugin.hostname.clone(), plugin.port), plugin.clone());

        // Cleanup old providers
        if let Some(old) = old {
            for service in old.info.services {
                for provider in service.providers {
                    if let Err(e) =
                        self.clean_service(service.r#type, provider, old.hostname.clone(), old.port)
                    {
                        warn!("Failed to clean old provider: {}", e);
                    }
                }
            }
        }

        // Register new providers
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

    fn clean_service(
        &self,
        service_type: i32,
        provider: String,
        hostname: String,
        port: u16,
    ) -> Result<()> {
        if let Some(set) = self
            .providers
            .get_mut(&(service_type.try_into()?, provider))
        {
            set.remove(&(hostname, port));
        }

        Ok(())
    }
}
