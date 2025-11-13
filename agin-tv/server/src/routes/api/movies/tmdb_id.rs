use axum::{Extension, Json, extract::Path};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    entity::{episode, movie},
    errors::AxumResult,
    state::AppState,
    util::{
        importer::{FetchPolicy, Importer, MovieDetails},
        tmdb::{id::TmdbId, import::TmdbImporter},
    },
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(search))
}

/// Movie Details
///
/// Get movie details by its TMDB ID
#[utoipa::path(
    method(get),
    path = "/",
    params(
        ("tmdb_id" = TmdbId, Path)
    ),
    responses(
        (status = OK, description = "Success", body = MovieDetails)
    ),
    tag = "Movie"
)]
async fn search(
    Extension(state): Extension<AppState>,
    Path(tmdb_id): Path<TmdbId>,
) -> AxumResult<Json<MovieDetails>> {
    let importer = TmdbImporter::new(state.tmdb.clone(), state.db.clone());

    let data = importer.import(tmdb_id, FetchPolicy::IfNotExists).await?;

    Ok(Json(data))
}
