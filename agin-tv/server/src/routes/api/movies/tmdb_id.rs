use axum::{
    Extension, Json,
    extract::{Path, Query},
};
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    errors::AxumResult,
    importer::{FetchPolicy, Importer, MovieDetails, tmdb::TmdbImporter},
    state::AppState,
    util::{opt_bool_from_string, tmdb::TmdbId},
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(search))
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Params {
    #[serde(default, deserialize_with = "crate::util::opt_bool_from_string")]
    pub refresh_cache: Option<bool>,
}

/// Movie Details
///
/// Get movie details by its TMDB ID
#[utoipa::path(
    method(get),
    path = "/",
    params(
        ("tmdb_id" = TmdbId, Path),
        Params
    ),
    responses(
        (status = OK, description = "Success", body = MovieDetails)
    ),
    tag = "Movie"
)]
async fn search(
    Extension(state): Extension<AppState>,
    Path(tmdb_id): Path<TmdbId>,
    Query(params): Query<Params>,
) -> AxumResult<Json<MovieDetails>> {
    let refresh_cache = params.refresh_cache.unwrap_or(false);
    let fetch_policy = if refresh_cache {
        FetchPolicy::Always
    } else {
        FetchPolicy::IfNotExists
    };

    let importer = TmdbImporter::new(state.tmdb.clone(), state.db.clone());

    let data = importer.import(tmdb_id, fetch_policy).await?;

    Ok(Json(data))
}
