use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::Deserialize;
use utoipa::ToSchema;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub user_id: i32,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,

    #[sea_orm(indexed, unique)]
    pub token_hash: String,

    pub device_name: String,

    pub device_type: DeviceType,

    pub platform: Platform,

    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum DeviceType {
    Mobile,
    Desktop,
    Tv,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum Platform {
    Android,
    Ios,
    Windows,
    Macos,
    Linux,
    Tv,
}

impl ActiveModelBehavior for ActiveModel {}
