mod discovery;
mod login;
mod search;

use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let auth = OpenApiRouter::new().nest("/search", search::routes());

    let public = OpenApiRouter::new()
        .nest("/discovery", discovery::routes())
        .nest("/login", login::routes());

    auth.merge(public)
}
