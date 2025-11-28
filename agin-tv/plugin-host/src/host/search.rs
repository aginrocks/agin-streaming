use plugin_proto::{SearchRequest, ServiceType};

use crate::host::PluginHost;

impl PluginHost {
    pub async fn search(&self, request: SearchRequest) {
        let mut results = Vec::new();

        for entry in self.plugins.iter() {
            let plugin = entry.value();
            if plugin
                .info
                .services
                .iter()
                .any(|s| s.r#type == ServiceType::Search as i32)
            {
                let mut services = plugin.clients.write().await;
                match services.search.search(request.clone()).await {
                    Ok(services) => {
                        let response = services.into_inner();
                        results.extend(response.results);
                    }
                    Err(e) => {
                        tracing::warn!(
                            "Search request to plugin {}:{} failed: {}",
                            plugin.hostname,
                            plugin.port,
                            e
                        );
                    }
                }
            }
        }
    }
}
