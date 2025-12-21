use axum::{Extension, Json, extract::Path};
use plugin_proto::{ResolvedSource, Source};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{errors::AxumResult, state::AppState, util::tmdb::TmdbId};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_source))
}

#[derive(Serialize, ToSchema)]
pub struct PlayerInfo {
    pub sources: Vec<ResolvedSource>,
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct SourceParams {
    pub tmdb_id: TmdbId,
    pub season: i32,
    pub episode: i32,
    pub source: String,
}

/// Source details
///
/// Get a source's details including direct playback links, subtitles, etc.
#[utoipa::path(
    method(get),
    path = "/",
    params(
        SourceParams,
    ),
    responses(
        (status = OK, description = "Success", body = PlayerInfo)
    ),
    tag = "Episode Source"
)]
async fn get_source(
    Extension(state): Extension<AppState>,
    Path(params): Path<SourceParams>,
) -> AxumResult<Json<PlayerInfo>> {
    todo!()
}
