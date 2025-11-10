use sea_orm::entity::prelude::*;

use crate::util::tmdb_id::TmdbId;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "movies")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub tmdb_id: TmdbId,
    pub r#type: ContentType,
    pub title: String,
    pub overview: String,
    pub tagline: Option<String>,
    pub poster: String,
    pub horizontal_poster: Option<String>,
    pub backdrop: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "content_type")]
pub enum ContentType {
    #[sea_orm(string_value = "movie")]
    Movie,
    #[sea_orm(string_value = "tv")]
    Tv,
}

impl ActiveModelBehavior for ActiveModel {}
