pub mod tmdb;

use async_trait::async_trait;
use sea_orm::DbErr;
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::{
    entity::{episode, movie},
    util::tmdb::id::TmdbId,
};

#[derive(Serialize, ToSchema)]
pub struct MovieDetails {
    #[serde(flatten)]
    pub movie: movie::Model,
    pub episodes: Vec<episode::Model>,
}

#[derive(Error, Debug)]
pub enum ImporterError {
    #[error("Database error")]
    DbErr(#[from] DbErr),

    #[error("API error")]
    ApiError(#[from] tmdb_api::error::Error),

    #[error("Unsupported media type")]
    UnsupportedMediaType,
}

#[derive(Default, PartialEq, Eq)]
pub enum FetchPolicy {
    #[default]
    IfNotExists,
    OnlyMissingEpisodes,
    Always,
}

#[async_trait]
pub trait Importer {
    /// Imports a movie or TV show from TMDB or updates it if it exists.
    async fn import(
        &self,
        tmdb_id: TmdbId,
        fetch_policy: FetchPolicy,
    ) -> Result<MovieDetails, ImporterError>;

    /// Updates existing movie or TV show details and episodes. Used internally by `import` method.
    async fn update(
        &self,
        tmdb_id: TmdbId,
        episodes: Vec<episode::Model>,
        fetch_policy: FetchPolicy,
    ) -> Result<MovieDetails, ImporterError>;
}
