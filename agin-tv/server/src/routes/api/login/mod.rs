use axum::{Extension, Json};
use axum_valid::Valid;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use validator::Validate;

use crate::{
    entity::token::{self, DeviceType, Platform},
    errors::AxumResult,
    settings::Oidc,
    state::AppState,
    util::tokens::{generate_token, hash_token},
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(login))
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, max = 2048))]
    pub oidc_token: String,

    #[validate(length(min = 1, max = 64))]
    pub device_name: String,

    pub device_type: DeviceType,

    pub platform: Platform,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

/// Log in
///
/// Authenticate with the OIDC provider specified in [`GET /api/discovery`](#tag/auth/get/api/discovery) and obtain an access token.
///
/// Exchange the OIDC token for an access token that can be used to authenticate subsequent requests.
#[utoipa::path(
    method(post),
    path = "/",
    responses(
        (status = OK, description = "Success", body = LoginResponse)
    ),
    tag = "Auth"
)]
async fn login(
    Extension(state): Extension<AppState>,
    Valid(Json(body)): Valid<Json<LoginRequest>>,
) -> AxumResult<Json<LoginResponse>> {
    // TODO: Validate user

    let token = generate_token();
    let hashed_token = hash_token(&token);

    let token_entry = token::ActiveModel {
        user_id: todo!(),
        token_hash: Set(hashed_token),
        device_name: Set(body.device_name),
        device_type: Set(body.device_type),
        platform: Set(body.platform),
        created_at: Set(Utc::now()),
        ..Default::default()
    };

    token_entry.insert(&state.db).await?;

    // TODO: Save the hashed token to the database along with device info

    Ok(Json(LoginResponse { token }))
}
