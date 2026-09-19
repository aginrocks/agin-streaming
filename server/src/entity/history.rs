use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(indexed)]
    pub user_id: i32,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,

    #[sea_orm(indexed)]
    pub movie_id: i32,
    #[sea_orm(belongs_to, from = "movie_id", to = "id")]
    pub movie: HasOne<super::movie::Entity>,

    #[sea_orm(indexed)]
    pub token_id: i32,
    #[sea_orm(belongs_to, from = "token_id", to = "id")]
    pub token: HasOne<super::token::Entity>,

    pub started_at: DateTime<Utc>,

    pub stopped_at: Option<DateTime<Utc>>,

    pub position_seconds: i32,

    pub duration_seconds: Option<i32>,

    pub completion_percentage: Decimal,
}

impl ActiveModelBehavior for ActiveModel {}
