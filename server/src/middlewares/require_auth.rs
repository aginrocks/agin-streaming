use axum::{Extension, extract::Request, middleware::Next, response::Response};
use color_eyre::eyre::eyre;
use sea_orm::EntityTrait;

use crate::{
    entity::user,
    errors::{AxumError, AxumResult},
    state::AppState,
};

pub async fn require_auth(
    Extension(state): Extension<AppState>,
    mut request: Request,
    next: Next,
) -> AxumResult<Response> {
    let user = user::Entity::find_by_email("test@test.test")
        .one(&state.db)
        .await?
        .ok_or_else(|| {
            AxumError::new(eyre!(
                "User test is hardcoded for now and should be present in the database"
            ))
        });

    Ok(next.run(request).await)
}
