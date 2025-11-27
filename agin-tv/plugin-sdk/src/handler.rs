use thiserror::Error;

use crate::{
    BoxFuture,
    context::Context,
    plugin::{
        GetSourcesRequest, GetSourcesResponse, SearchRequest, SearchResponse, Service, ServiceType,
    },
};

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

#[derive(Clone)]
pub struct HandlerMetadata<T: Send + Sync + Clone + 'static> {
    pub providers: Vec<String>,

    pub info: HandlerInfo<T>,
}

impl<T: Send + Sync + Clone + 'static> From<HandlerMetadata<T>> for Service {
    fn from(value: HandlerMetadata<T>) -> Self {
        Self {
            r#type: match value.info {
                HandlerInfo::Search { .. } => ServiceType::Search.into(),
                HandlerInfo::SourceProvider { .. } => ServiceType::SourceProvider.into(),
            },
            providers: value.providers,
        }
    }
}

#[derive(Clone)]
pub enum HandlerInfo<T: Send + Sync + Clone + 'static> {
    Search(SearchHandler<T>),
    SourceProvider(SourceProviderHandler<T>),
}

#[derive(Clone)]
pub struct SearchHandler<T: Send + Sync + Clone + 'static> {
    pub callback:
        fn(Context<T>, SearchRequest) -> BoxFuture<'static, Result<SearchResponse, HandlerError>>,
}

#[derive(Clone)]
pub struct SourceProviderHandler<T: Send + Sync + Clone + 'static> {
    pub callback: fn(
        Context<T>,
        GetSourcesRequest,
    ) -> BoxFuture<'static, Result<GetSourcesResponse, HandlerError>>,
}
