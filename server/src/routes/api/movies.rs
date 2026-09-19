mod tmdb_id;

use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().nest("/{tmdb_id}", tmdb_id::routes())
}
