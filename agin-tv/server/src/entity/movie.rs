use sea_orm::entity::prelude::*;
use serde::Serialize;
use thiserror::Error;
use tmdb_api::multi::MultiSearchResult;
use utoipa::ToSchema;

use crate::util::tmdb_id::TmdbId;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, ToSchema)]
#[sea_orm(table_name = "movies")]
#[schema(title = "Movie")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(indexed, unique)]
    pub tmdb_id: TmdbId,
    pub r#type: ContentType,
    pub title: String,
    pub overview: String,
    pub tagline: Option<String>,
    pub poster: String,
    pub horizontal_poster: Option<String>,
    pub backdrop: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, ToSchema)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "content_type")]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    #[sea_orm(string_value = "movie")]
    Movie,
    #[sea_orm(string_value = "tv")]
    Tv,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Error, Debug)]
#[error("Only `Movie` and `TVShow` variants can be converted to `Movie` entity")]
pub struct MultiSearchConvertError;

impl TryFrom<MultiSearchResult> for Model {
    type Error = MultiSearchConvertError;

    fn try_from(value: MultiSearchResult) -> Result<Self, Self::Error> {
        match value {
            MultiSearchResult::Movie(movie) => Ok(Model {
                id: 0,
                tmdb_id: TmdbId::Movie(movie.inner.id as i32),
                backdrop: movie.inner.backdrop_path.unwrap_or_default(),
                poster: movie.inner.poster_path.unwrap_or_default(),
                horizontal_poster: None,
                overview: movie.inner.overview,
                title: movie.inner.title,
                tagline: None,
                r#type: ContentType::Movie,
            }),
            MultiSearchResult::TVShow(tv) => Ok(Model {
                id: 0,
                tmdb_id: TmdbId::TvShow(tv.inner.id as i32),
                backdrop: tv.inner.backdrop_path.unwrap_or_default(),
                poster: tv.inner.poster_path.unwrap_or_default(),
                horizontal_poster: None,
                overview: tv.inner.overview.unwrap_or_default(),
                title: tv.inner.name,
                tagline: None,
                r#type: ContentType::Tv,
            }),
            MultiSearchResult::Person(_) => Err(MultiSearchConvertError),
        }
    }
}
