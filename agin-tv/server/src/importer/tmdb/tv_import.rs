use crate::{
    entity::episode,
    importer::{FetchPolicy, ImporterError, MovieDetails},
};

impl super::TmdbImporter {
    pub async fn update_tv_show(
        &self,
        tmdb_id: i32,
        episodes: Vec<episode::Model>,
        fetch_policy: FetchPolicy,
    ) -> Result<MovieDetails, ImporterError> {
        todo!()
    }
}
