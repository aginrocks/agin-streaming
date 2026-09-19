mod source;

use axum::{Extension, Json, extract::Path};
use plugin_proto::Source;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{errors::AxumResult, state::AppState, util::tmdb::TmdbId};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_sources))
        .nest("/{source}", source::routes())
}

#[derive(Serialize, ToSchema)]
pub struct PlayerInfo {
    pub sources: Vec<Source>,
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct SourcesParams {
    pub tmdb_id: TmdbId,
    pub season: i32,
    pub episode: i32,
}

/// Sources
///
/// Get a list of avalibale sources for the given episode
#[utoipa::path(
    method(get),
    path = "/",
    params(
        SourcesParams
    ),
    responses(
        (status = OK, description = "Success", body = PlayerInfo)
    ),
    tag = "Episode"
)]
async fn get_sources(
    Extension(state): Extension<AppState>,
    Path(params): Path<SourcesParams>,
) -> AxumResult<Json<PlayerInfo>> {
    todo!()
}
