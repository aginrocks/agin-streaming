use sea_orm::{ActiveValue::Set, entity::prelude::*};
use serde::Serialize;
use utoipa::ToSchema;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, ToSchema)]
#[sea_orm(table_name = "episodes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique_key = "movie")]
    pub movie_id: i32,
    #[schema(value_type = ())]
    #[sea_orm(belongs_to, from = "movie_id", to = "id")]
    pub movie: HasOne<super::movie::Entity>,
    pub name: String,
    pub overview: String,
    #[sea_orm(unique_key = "movie")]
    pub season_number: i32,
    #[sea_orm(unique_key = "movie")]
    pub episode_number: i32,
    pub poster: Option<String>,
    pub runtime: Option<i32>,
}

impl ActiveModelBehavior for ActiveModel {}

pub static IDENTIFYING_COLUMNS: [Column; 3] =
    [Column::MovieId, Column::SeasonNumber, Column::EpisodeNumber];

pub static UPDATABLE_COLUMNS: [Column; 4] = [
    Column::Name,
    Column::Overview,
    Column::Poster,
    Column::Runtime,
];

/// Converts TMDB Episode to Episode ActiveModel
/// **Remember to set `movie_id` before inserting the model**
impl From<tmdb_api::tvshow::Episode> for ActiveModel {
    fn from(value: tmdb_api::tvshow::Episode) -> Self {
        Self {
            name: Set(value.inner.name),
            overview: Set(value.inner.overview.unwrap_or_default()),
            season_number: Set(value.inner.season_number as i32),
            episode_number: Set(value.inner.episode_number as i32),
            poster: Set(value.inner.still_path),
            // TODO
            runtime: Set(None),
            ..Default::default()
        }
    }
}
