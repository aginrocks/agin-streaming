use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "episodes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub movie_id: i32,
    #[sea_orm(belongs_to, from = "movie_id", to = "id")]
    pub movie: HasOne<super::movie::Entity>,
    pub name: String,
    pub overview: String,
    pub season_number: i32,
    pub episode_number: i32,
    pub poster: String,
    pub runtime: Option<i32>,
}

impl ActiveModelBehavior for ActiveModel {}
