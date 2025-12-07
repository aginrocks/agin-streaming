use color_eyre::Report;
use plugin_proto::{
    GetSourcesRequest, GetSourcesResponse, ResolveRequest, ResolveResponse, ResolveSourceRequest,
    ResolveSourceResponse, SearchRequest, SearchResponse, Service, ServiceType,
};
use thiserror::Error;

use crate::{BoxFuture, context::Context};

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    Other(#[from] Report),
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
                HandlerInfo::SourceResolver { .. } => ServiceType::SourceResolver.into(),
                HandlerInfo::LinkResolver { .. } => ServiceType::LinkResolver.into(),
            },
            providers: value.providers,
        }
    }
}

#[derive(Clone)]
pub enum HandlerInfo<T: Send + Sync + Clone + 'static> {
    Search(SearchHandler<T>),
    SourceProvider(SourceProviderHandler<T>),
    SourceResolver(SourceResolverHandler<T>),
    LinkResolver(LinkResolverHandler<T>),
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

#[derive(Clone)]
pub struct SourceResolverHandler<T: Send + Sync + Clone + 'static> {
    pub callback: fn(
        Context<T>,
        ResolveSourceRequest,
    ) -> BoxFuture<'static, Result<ResolveSourceResponse, HandlerError>>,
}

#[derive(Clone)]
pub struct LinkResolverHandler<T: Send + Sync + Clone + 'static> {
    pub callback:
        fn(Context<T>, ResolveRequest) -> BoxFuture<'static, Result<ResolveResponse, HandlerError>>,
}
