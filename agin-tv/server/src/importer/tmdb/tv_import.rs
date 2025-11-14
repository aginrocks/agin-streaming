use futures::future::join_all;
use sea_orm::{ActiveValue::Set, EntityTrait, sea_query::OnConflict};
use tmdb_api::tvshow::{Season, TVShow};
use tokio::try_join;
use tracing::{debug, instrument};

use crate::{
    entity::{episode, movie},
    importer::{ImporterError, MovieDetails},
};

// TODO: Handle season 0
impl super::TmdbImporter {
    #[instrument(skip(self, details))]
    pub async fn update_tv_show(
        &self,
        tmdb_id: i32,
        details: Option<movie::Model>,
    ) -> Result<MovieDetails, ImporterError> {
        const ADDITIONAL_SEASONS: usize = 1;

        // We guess the number of seasons to determine needed iterations before we know full details
        let seasons_guess =
            details.and_then(|d| d.seasons).unwrap_or(0) as usize + ADDITIONAL_SEASONS;

        debug!("Guessing {seasons_guess}");

        // Fetch metadata and new seasons concurrently
        let fetch_metadata = async {
            let details = self.fetch_tv_details(tmdb_id).await?;

            // There are seasons we haven't accounted for yet
            let seasons = if details.seasons.len() > seasons_guess {
                debug!(
                    "Unaccounted seasons found ({}..={})",
                    seasons_guess + 1,
                    details.seasons.len()
                );
                let jobs = ((seasons_guess + 1)..=details.seasons.len())
                    .map(async |s| self.fetch_season(tmdb_id, s as i32).await);

                join_all(jobs)
                    .await
                    .into_iter()
                    .filter_map(Result::ok)
                    .collect()
            } else {
                debug!("No unaccounted seasons");
                vec![]
            };

            Ok::<(TVShow, Vec<Season>), ImporterError>((details, seasons))
        };

        let fetch_known_seasons = async {
            debug!("Fetching known seasons (1..={seasons_guess})");
            let jobs =
                (1..=seasons_guess).map(async |s| self.fetch_season(tmdb_id, s as i32).await);

            Ok::<Vec<Season>, ImporterError>(
                join_all(jobs)
                    .await
                    .into_iter()
                    .filter_map(Result::ok)
                    .collect(),
            )
        };

        let ((details, new_seasons), known_seasons) =
            try_join!(fetch_metadata, fetch_known_seasons)?;

        let movie = self.write_details(details.clone().into()).await?;

        let seasons = [new_seasons.as_slice(), known_seasons.as_slice()].concat();
        let mut episodes = seasons
            .into_iter()
            .flat_map(|s| s.episodes)
            .collect::<Vec<_>>();

        episodes.sort_by(|a, b| {
            a.inner
                .season_number
                .cmp(&b.inner.season_number)
                .then(a.inner.episode_number.cmp(&b.inner.episode_number))
        });

        let episodes = episodes
            .into_iter()
            .map(|e| {
                let model = e.into();
                episode::ActiveModel {
                    movie_id: Set(movie.id),
                    ..model
                }
            })
            .collect::<Vec<episode::ActiveModel>>();

        let episodes = self.write_episodes(episodes).await?;

        Ok(MovieDetails { movie, episodes })
    }

    /// Fetches details from TMDB
    async fn fetch_tv_details(&self, tmdb_id: i32) -> Result<TVShow, ImporterError> {
        let details = self
            .tmdb
            .get_tvshow_details(tmdb_id as u64, &Default::default())
            .await?;

        Ok(details)
    }

    async fn fetch_season(
        &self,
        tmdb_id: i32,
        season_number: i32,
    ) -> Result<Season, ImporterError> {
        let params = Default::default();
        let details = self
            .tmdb
            .get_tvshow_season_details(tmdb_id as u64, season_number as u64, &params)
            .await?;

        Ok(details)
    }

    /// Writes movie details to the database
    async fn write_details(
        &self,
        details: movie::ActiveModel,
    ) -> Result<movie::Model, ImporterError> {
        let inserted_movie = movie::Entity::insert(details)
            .on_conflict(
                OnConflict::column(movie::Column::TmdbId)
                    .update_columns(movie::UPDATABLE_COLUMNS)
                    .to_owned(),
            )
            .exec_with_returning(&self.db)
            .await?;

        Ok(inserted_movie)
    }

    /// Writes episodes to the database
    async fn write_episodes(
        &self,
        episodes: Vec<episode::ActiveModel>,
    ) -> Result<Vec<episode::Model>, ImporterError> {
        let inserted_episodes = episode::Entity::insert_many(episodes)
            .on_conflict(
                OnConflict::columns(episode::IDENTIFYING_COLUMNS)
                    .update_columns(episode::UPDATABLE_COLUMNS)
                    .to_owned(),
            )
            .exec_with_returning(&self.db)
            .await?;

        Ok(inserted_episodes)
    }
}
