use std::sync::Arc;

use futures::future::join_all;
use tonic::{Request, Response, Status};

use crate::{
    handler::HandlerInfo,
    plugin::{ResolveRequest, ResolveResponse, link_resolver_service_server::LinkResolverService},
    sdk::PluginSdk,
};

#[derive(Clone)]
pub struct LinkResolver<T: Send + Sync + Clone + 'static> {
    pub sdk: Arc<PluginSdk<T>>,
}

#[tonic::async_trait]
impl<T: Send + Sync + Clone + 'static> LinkResolverService for LinkResolver<T> {
    async fn resolve(
        &self,
        request: Request<ResolveRequest>,
    ) -> Result<Response<ResolveResponse>, Status> {
        let matching_services = self.sdk.services.iter().filter_map(|s| {
            let HandlerInfo::LinkResolver(link_resolver) = &s.info else {
                return None;
            };
            if s.providers.contains(&request.get_ref().provider_id) {
                Some(link_resolver)
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

        let resolved_response = ResolveResponse { sources: results };

        Ok(Response::new(resolved_response))
    }
}
