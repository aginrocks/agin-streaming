mod discovery;
mod login;
mod movies;
mod search;

use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let auth = OpenApiRouter::new()
        .nest("/search", search::routes())
        .nest("/movies", movies::routes());

    let public = OpenApiRouter::new()
        .nest("/discovery", discovery::routes())
        .nest("/login", login::routes());

    auth.merge(public)
}
