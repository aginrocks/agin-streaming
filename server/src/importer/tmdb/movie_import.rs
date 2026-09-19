use sea_orm::{ActiveValue::Set, EntityTrait, sea_query::OnConflict};

use crate::{
    entity::{episode, movie},
    importer::{ImporterError, MovieDetails},
};

impl super::TmdbImporter {
    pub async fn update_movie(&self, tmdb_id: i32) -> Result<MovieDetails, ImporterError> {
        let details = self
            .tmdb
            .get_movie_details(tmdb_id as u64, &Default::default())
            .await?;

        let model: movie::ActiveModel = details.clone().into();
        let inserted_movie = movie::Entity::insert(model)
            .on_conflict(
                OnConflict::column(movie::Column::TmdbId)
                    .update_columns(movie::UPDATABLE_COLUMNS)
                    .to_owned(),
            )
            .exec_with_returning(&self.db)
            .await?;

        let episode_model = episode::ActiveModel {
            movie_id: Set(inserted_movie.id),
            name: Set(details.inner.title),
            overview: Set(details.inner.overview),
            season_number: Set(0),
            episode_number: Set(0),
            poster: Set(details.inner.backdrop_path),
            runtime: Set(details.runtime.map(|r| r as i32)),
            ..Default::default()
        };
        let inserted_episode = episode::Entity::insert(episode_model)
            .on_conflict(
                OnConflict::columns(episode::IDENTIFYING_COLUMNS)
                    .update_columns(episode::UPDATABLE_COLUMNS)
                    .to_owned(),
            )
            .exec_with_returning(&self.db)
            .await?;

        Ok(MovieDetails {
            movie: inserted_movie,
            episodes: vec![inserted_episode],
        })
    }
}
