use std::sync::Arc;

use futures::future::join_all;
use plugin_proto::{
    GetSourcesRequest, GetSourcesResponse, ResolveSourceRequest, ResolveSourceResponse,
    source_provider_service_server::SourceProviderService,
};
use tonic::{Request, Response, Status};
use tracing::info;

use crate::{handler::HandlerInfo, sdk::PluginSdk};

#[derive(Clone)]
pub struct SourceProvider<T: Send + Sync + Clone + 'static> {
    pub sdk: Arc<PluginSdk<T>>,
}

#[tonic::async_trait]
impl<T: Send + Sync + Clone + 'static> SourceProviderService for SourceProvider<T> {
    async fn get_sources(
        &self,
        request: Request<GetSourcesRequest>,
    ) -> Result<Response<GetSourcesResponse>, Status> {
        info!("Received request");
        let matching_services = self.sdk.services.iter().filter_map(|s| {
            let HandlerInfo::SourceProvider(source_provider) = &s.info else {
                return None;
            };
            if s.providers.contains(&request.get_ref().provider_id) {
                Some(source_provider)
            } else {
                None
            }
        });

        let handles = matching_services.map(|service| {
            tokio::spawn({
                (service.callback)(self.sdk.context.clone(), request.get_ref().clone())
            })
        });

        let results = join_all(handles).await;

        let results = results
            .into_iter()
            .filter_map(|res| match res {
                Ok(Ok(response)) => Some(response.sources),
                _ => None,
            })
            .collect::<Vec<_>>()
            .concat();

        let sources_response = GetSourcesResponse { sources: results };

        Ok(Response::new(sources_response))
    }

    async fn resolve_source(
        &self,
        request: Request<ResolveSourceRequest>,
    ) -> Result<Response<ResolveSourceResponse>, Status> {
        info!("Received request");
        let matching_services = self.sdk.services.iter().filter_map(|s| {
            let HandlerInfo::SourceResolver(source_resovler) = &s.info else {
                return None;
            };
            if s.providers.contains(&request.get_ref().provider_id) {
                Some(source_resovler)
            } else {
                None
            }
        });

        let handles = matching_services.map(|service| {
            tokio::spawn({
                (service.callback)(self.sdk.context.clone(), request.get_ref().clone())
            })
        });

        let results = join_all(handles).await;

        let results = results
            .into_iter()
            .filter_map(|res| match res {
                Ok(Ok(response)) => Some(response.sources),
                _ => None,
            })
            .collect::<Vec<_>>()
            .concat();

        let sources_response = ResolveSourceResponse { sources: results };

        Ok(Response::new(sources_response))
    }
}
