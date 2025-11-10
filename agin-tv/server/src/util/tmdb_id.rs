use std::fmt::Display;

use sea_orm::entity::prelude::*;
use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(Debug, Clone, PartialEq, Eq, DeriveValueType, SerializeDisplay, DeserializeFromStr)]
#[sea_orm(value_type = "String")]
pub enum TmdbId {
    Movie(i32),
    TvShow(i32),
    Custom(i32),
}

impl std::str::FromStr for TmdbId {
    type Err = sea_orm::sea_query::ValueTypeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prefix, id) = s.split_at(1);
        let parsed_id = id.parse().map_err(|_| sea_orm::sea_query::ValueTypeErr)?;

        match prefix {
            "m" => Ok(TmdbId::Movie(parsed_id)),
            "t" => Ok(TmdbId::TvShow(parsed_id)),
            "c" => Ok(TmdbId::Custom(parsed_id)),
            _ => Err(sea_orm::sea_query::ValueTypeErr),
        }
    }
}

impl Display for TmdbId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TmdbId::Movie(id) => write!(f, "m{id}"),
            TmdbId::TvShow(id) => write!(f, "t{id}"),
            TmdbId::Custom(id) => write!(f, "c{id}"),
        }
    }
}
