// We don't have enough time to implement proper login flow with OIDC as we wanted so we just create one test user if it's not in the database already.

use color_eyre::eyre::Result;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

use crate::{entity::user, state::AppState};

pub async fn demo_bootstrap(state: &AppState) -> Result<()> {
    if user::Entity::find_by_email("test@test.test")
        .one(&state.db)
        .await?
        .is_none()
    {
        user::ActiveModel {
            email: Set("test@test.test".to_string()),
            role: Set(user::Role::User),
            username: Set("test".to_string()),
            subject: Set("placeholder".to_string()),
            ..Default::default()
        }
        .insert(&state.db)
        .await?;
    }

    Ok(())
}
