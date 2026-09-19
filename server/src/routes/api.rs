mod discovery;
mod login;
mod movies;
mod search;

use axum::middleware;
use utoipa_axum::router::OpenApiRouter;

use crate::{middlewares::require_auth::require_auth, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    let auth = OpenApiRouter::new()
        .nest("/search", search::routes())
        .nest("/movies", movies::routes())
        .layer(middleware::from_fn(require_auth));

    let public = OpenApiRouter::new()
        .nest("/discovery", discovery::routes())
        .nest("/login", login::routes());

    auth.merge(public)
}
