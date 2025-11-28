use futures::future::join_all;
use plugin_proto::{SearchRequest, SearchResult, ServiceType};
use tonic::Status;

use crate::{host::PluginHost, plugin::Plugin};

impl PluginHost {
    pub async fn search(&self, request: SearchRequest) -> Vec<SearchResult> {
        let jobs = self
            .plugins
            .iter()
            .map(|entry| {
                let plugin = entry.value();
                self.dispatch_search(plugin.clone(), request.clone())
            })
            .collect::<Vec<_>>();

        join_all(jobs)
            .await
            .into_iter()
            .filter_map(|r| r.ok())
            .flatten()
            .collect()
    }

    async fn dispatch_search(
        &self,
        plugin: Plugin,
        request: SearchRequest,
    ) -> Result<Vec<SearchResult>, Status> {
        if plugin
            .info
            .services
            .iter()
            .any(|s| s.r#type == ServiceType::Search as i32)
        {
            let mut services = plugin.clients.write().await;
            let result = services.search.search(request.clone()).await;

            if let Err(e) = &result {
                tracing::warn!(
                    "Search request to plugin {}:{} failed: {}",
                    plugin.hostname,
                    plugin.port,
                    e
                );
            }

            result.map(|res| res.into_inner().results)
        } else {
            Ok(Vec::new())
        }
    }
}
