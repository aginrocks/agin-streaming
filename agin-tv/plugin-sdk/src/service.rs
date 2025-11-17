use thiserror::Error;

use crate::plugin::{SearchRequest, SearchResponse, Service};

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

pub trait AginService {
    fn metadata(&self) -> Service;
}

pub trait SearchService {
    // fn search(request: SearchRequest) -> Result<SearchResponse>;
}
