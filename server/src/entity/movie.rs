use sea_orm::{ActiveValue::Set, entity::prelude::*};
use serde::Serialize;
use thiserror::Error;
use tmdb_api::multi::MultiSearchResult;
use utoipa::ToSchema;

use crate::util::tmdb::id::TmdbId;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, ToSchema)]
#[sea_orm(table_name = "movies")]
#[schema(title = "Movie")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique_key = "tmdb_id")]
    pub tmdb_id: TmdbId,
    pub r#type: ContentType,
    pub title: String,
    pub overview: String,
    pub tagline: Option<String>,
    pub poster: Option<String>,
    pub horizontal_poster: Option<String>,
    pub backdrop: Option<String>,
    pub seasons: Option<i32>,
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

pub static UPDATABLE_COLUMNS: [Column; 6] = [
    Column::Title,
    Column::Overview,
    Column::Tagline,
    Column::Poster,
    Column::HorizontalPoster,
    Column::Backdrop,
];

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
                backdrop: movie.inner.backdrop_path,
                poster: movie.inner.poster_path,
                horizontal_poster: None,
                overview: movie.inner.overview,
                title: movie.inner.title,
                tagline: None,
                r#type: ContentType::Movie,
                seasons: None,
            }),
            MultiSearchResult::TVShow(tv) => Ok(Model {
                id: 0,
                tmdb_id: TmdbId::TvShow(tv.inner.id as i32),
                backdrop: tv.inner.backdrop_path,
                poster: tv.inner.poster_path,
                horizontal_poster: None,
                overview: tv.inner.overview.unwrap_or_default(),
                title: tv.inner.name,
                tagline: None,
                r#type: ContentType::Tv,
                seasons: None,
            }),
            MultiSearchResult::Person(_) => Err(MultiSearchConvertError),
        }
    }
}

impl From<tmdb_api::movie::Movie> for ActiveModel {
    fn from(value: tmdb_api::movie::Movie) -> Self {
        Self {
            tmdb_id: Set(TmdbId::Movie(value.inner.id as i32)),
            r#type: Set(ContentType::Movie),
            title: Set(value.inner.title),
            overview: Set(value.inner.overview),
            tagline: Set(value.tagline),
            poster: Set(value.inner.poster_path),
            // TODO: import from `images`
            horizontal_poster: Set(None),
            backdrop: Set(value.inner.backdrop_path),
            ..Default::default()
        }
    }
}

impl From<tmdb_api::tvshow::TVShow> for ActiveModel {
    fn from(value: tmdb_api::tvshow::TVShow) -> Self {
        Self {
            tmdb_id: Set(TmdbId::TvShow(value.inner.id as i32)),
            r#type: Set(ContentType::Tv),
            title: Set(value.inner.name),
            overview: Set(value.inner.overview.unwrap_or_default()),
            tagline: Set(value.tagline),
            poster: Set(value.inner.poster_path),
            // TODO
            horizontal_poster: Set(None),
            backdrop: Set(value.inner.backdrop_path),
            seasons: Set(Some(value.seasons.len() as i32)),
            ..Default::default()
        }
    }
}
