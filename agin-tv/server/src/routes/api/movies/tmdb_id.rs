use axum::{Extension, Json, extract::Path};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    errors::AxumResult,
    importer::{FetchPolicy, Importer, MovieDetails, tmdb::TmdbImporter},
    state::AppState,
    util::tmdb::TmdbId,
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
