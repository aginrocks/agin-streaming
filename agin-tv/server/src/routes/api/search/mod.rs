use axum::{Extension, Json, extract::Query};
use axum_valid::Valid;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use validator::Validate;

use crate::{entity::movie, errors::AxumResult, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(search))
}

#[derive(Deserialize, Validate, IntoParams)]
pub struct SearchParams {
    pub query: String,
}

#[derive(Serialize, ToSchema)]
pub struct SearchResults {
    pub results: Vec<movie::Model>,
}

/// Search
///
/// Search movies and TV shows.
///
/// Search results are provided by:
/// - [The Movie Database (TMDB)](https://www.themoviedb.org/)
/// - Custom extensions installed by the user (for example the official file system extension)
///
/// > [!WARNING]
/// > Some fields are marked as optional, because at the time of querying TMDB they may not be available.
/// > For example, if your search query matches a movie that wasn't previously viewed by anyone using the instance, some fields may be unavaliable until you request more details about that movie.
/// > In that case, **ID will be set to `0`**
#[utoipa::path(
    method(get),
    path = "/",
    params(
        SearchParams
    ),
    responses(
        (status = OK, description = "Success", body = SearchResults)
    ),
    tag = "Movies"
)]
async fn search(
    Extension(state): Extension<AppState>,
    Valid(params): Valid<Query<SearchParams>>,
) -> AxumResult<Json<SearchResults>> {
    let results = state
        .tmdb
        .search_multi(params.query.clone(), &Default::default())
        .await?;

    let results = results
        .results
        .into_iter()
        .map(movie::Model::try_from)
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>();

    Ok(Json(SearchResults { results }))
}
