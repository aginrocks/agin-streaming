use std::collections::HashMap;

use axum::{Extension, Json, extract::Query};
use axum_valid::Valid;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use validator::Validate;

use crate::{entity::movie, errors::AxumResult, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(search))
}

#[derive(Deserialize, Validate, IntoParams)]
#[into_params(parameter_in = Query)]
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
/// > For example, if your search query matches a movie that wasn't previously viewed by anyone using the instance,
/// > some fields may be unavaliable until you request more details about that movie.
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

    let mut converted = Vec::with_capacity(results.results.len());
    let mut tmdb_ids = Vec::with_capacity(results.results.len());

    for result in results.results {
        if let Ok(model) = movie::Model::try_from(result) {
            tmdb_ids.push(model.tmdb_id);
            converted.push(model);
        }
    }

    let existing_movies = movie::Entity::find()
        .filter(movie::Column::TmdbId.is_in(tmdb_ids.clone()))
        .all(&state.db)
        .await?;

    let existing_map = existing_movies
        .into_iter()
        .map(|m| (m.tmdb_id, m))
        .collect::<HashMap<_, _>>();

    let merged_results = converted
        .into_iter()
        .map(|m| existing_map.get(&m.tmdb_id).cloned().unwrap_or(m))
        .collect::<Vec<_>>();

    Ok(Json(SearchResults {
        results: merged_results,
    }))
}
