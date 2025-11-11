mod discovery;
mod login;

use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let auth = OpenApiRouter::new();

    let public = OpenApiRouter::new()
        .nest("/discovery", discovery::routes())
        .nest("/login", login::routes());

    auth.merge(public)
}
