use std::fmt::Display;

use sea_orm::entity::prelude::*;
use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(Debug, Clone, PartialEq, Eq, DeriveValueType, SerializeDisplay, DeserializeFromStr)]
#[sea_orm(value_type = "String")]
pub enum TmdbId {
    Movie(i32),
    TvShow(i32),
    Custom(String),
}

impl std::str::FromStr for TmdbId {
    type Err = sea_orm::sea_query::ValueTypeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix('m') {
            Ok(TmdbId::Movie(
                stripped
                    .parse()
                    .map_err(|_| sea_orm::sea_query::ValueTypeErr)?,
            ))
        } else if let Some(stripped) = s.strip_prefix('t') {
            Ok(TmdbId::TvShow(
                stripped
                    .parse()
                    .map_err(|_| sea_orm::sea_query::ValueTypeErr)?,
            ))
        } else if let Some(stripped) = s.strip_prefix('c') {
            Ok(TmdbId::Custom(stripped.to_string()))
        } else {
            Err(sea_orm::sea_query::ValueTypeErr)
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
