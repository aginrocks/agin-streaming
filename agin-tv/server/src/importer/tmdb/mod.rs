pub mod movie_import;
pub mod tv_import;

use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

use crate::{
    entity::{episode, movie},
    importer::{FetchPolicy, Importer, ImporterError, MovieDetails},
    util::tmdb::id::TmdbId,
};

pub struct TmdbImporter {
    tmdb: Arc<tmdb_api::client::ReqwestClient>,
    db: DatabaseConnection,
}

impl TmdbImporter {
    pub fn new(tmdb: Arc<tmdb_api::client::ReqwestClient>, db: DatabaseConnection) -> Self {
        Self { tmdb, db }
    }
}

#[async_trait]
impl Importer for TmdbImporter {
    async fn import(
        &self,
        tmdb_id: TmdbId,
        fetch_policy: FetchPolicy,
    ) -> Result<MovieDetails, ImporterError> {
        let details = movie::Entity::find_by_tmdb_id(tmdb_id)
            .one(&self.db)
            .await?;

        if let Some(details) = details {
            let episodes = episode::Entity::find()
                .filter(episode::Column::MovieId.eq(details.id))
                .order_by_asc(episode::Column::SeasonNumber)
                .order_by_asc(episode::Column::EpisodeNumber)
                .all(&self.db)
                .await?;

            match fetch_policy {
                FetchPolicy::IfNotExists => Ok(MovieDetails {
                    movie: details,
                    episodes,
                }),
                _ => self.update(tmdb_id, Some(details)).await,
            }
        } else {
            self.update(tmdb_id, None).await
        }
    }

    async fn update(
        &self,
        tmdb_id: TmdbId,
        details: Option<movie::Model>,
    ) -> Result<MovieDetails, ImporterError> {
        match tmdb_id {
            TmdbId::Movie(id) => self.update_movie(id).await,
            TmdbId::TvShow(id) => self.update_tv_show(id, details).await,
            _ => Err(ImporterError::UnsupportedMediaType),
        }
    }
}
