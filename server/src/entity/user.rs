use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub role: Role,
    pub subject: String,
    pub username: String,
    pub email: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "user_role",
    rename_all = "lowercase"
)]
pub enum Role {
    #[sea_orm(string_value = "user")]
    User,
    #[sea_orm(string_value = "admin")]
    Admin,
}

impl ActiveModelBehavior for ActiveModel {}
