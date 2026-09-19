mod episode;

use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().nest("/{episode}", episode::routes())
}
