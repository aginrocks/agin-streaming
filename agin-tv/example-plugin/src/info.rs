use plugin_sdk::{
    context::Context,
    plugin::{SearchRequest, SearchResponse},
    service::{ServiceError, ServiceInfo},
};

use crate::State;

pub fn info() -> ServiceInfo<State> {
    async fn inner(
        ctx: Context<State>,
        request: SearchRequest,
    ) -> Result<SearchResponse, ServiceError> {
        todo!()
    }

    ServiceInfo::Search {
        callback: |ctx, request| Box::pin(async move { inner(ctx, request).await }),
    }
}
