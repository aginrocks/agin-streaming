use thiserror::Error;

use crate::{
    BoxFuture,
    context::Context,
    plugin::{SearchRequest, SearchResponse, Service},
};

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

pub struct ServiceMetadata<T: Send + Sync + 'static> {
    pub providers: Vec<&'static str>,

    pub info: ServiceInfo<T>,
}

pub enum ServiceInfo<T: Send + Sync + 'static> {
    Search {
        callback: fn(
            Context<T>,
            SearchRequest,
        ) -> BoxFuture<'static, Result<SearchResponse, ServiceError>>,
    },
}
