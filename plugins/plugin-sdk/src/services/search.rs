use std::sync::Arc;

use futures::future::join_all;
use plugin_proto::{SearchRequest, SearchResponse, search_service_server::SearchService};
use tonic::{Request, Response, Status};
use tracing::info;

use crate::{handler::HandlerInfo, sdk::PluginSdk};

#[derive(Clone)]
pub struct Search<T: Send + Sync + Clone + 'static> {
    pub sdk: Arc<PluginSdk<T>>,
}

#[tonic::async_trait]
impl<T: Send + Sync + Clone + 'static> SearchService for Search<T> {
    async fn search(
        &self,
        request: Request<SearchRequest>,
    ) -> Result<Response<SearchResponse>, Status> {
        info!("Received request");
        let search_services = self.sdk.services.iter().filter_map(|s| {
            let HandlerInfo::Search(search) = &s.info else {
                return None;
            };
            Some(search)
        });

        let handles = search_services.map(|service| {
            tokio::spawn({
                (service.callback)(self.sdk.context.clone(), request.get_ref().clone())
            })
        });

        let results = join_all(handles).await;

        let results = results
            .into_iter()
            .filter_map(|res| match res {
                Ok(Ok(response)) => Some(response.results),
                _ => None,
            })
            .collect::<Vec<_>>()
            .concat();

        let search_response = SearchResponse { results };

        Ok(Response::new(search_response))
    }
}
