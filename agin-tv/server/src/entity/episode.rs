use sea_orm::entity::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, ToSchema)]
#[sea_orm(table_name = "episodes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(indexed)]
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
