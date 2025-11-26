use plugin_sdk::{
    context::Context,
    handler::{HandlerError, HandlerInfo, HandlerMetadata, SearchHandler},
    plugin::{SearchRequest, SearchResponse},
};

use crate::State;

pub fn info() -> HandlerMetadata<State> {
    async fn inner(
        ctx: Context<State>,
        request: SearchRequest,
    ) -> Result<SearchResponse, HandlerError> {
        todo!()
    }

    HandlerMetadata {
        providers: vec!["example-provider".to_string()],
        info: HandlerInfo::Search(SearchHandler {
            callback: |ctx, request| Box::pin(async move { inner(ctx, request).await }),
        }),
    }
}
