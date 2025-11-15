use axum::{Extension, Json};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{settings::Oidc, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(discovery))
}

#[derive(Serialize, ToSchema)]
pub struct DiscoveryResponse {
    pub agin_tv_instance: bool,

    pub version: String,

    pub fqdn: String,

    pub oidc: Oidc,
}

/// Discovery
///
/// Discover an instance of Agin TV and obtain OIDC configuration.
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = DiscoveryResponse)
    ),
    tag = "Auth"
)]
async fn discovery(Extension(state): Extension<AppState>) -> Json<DiscoveryResponse> {
    Json(DiscoveryResponse {
        agin_tv_instance: true,
        version: env!("CARGO_PKG_VERSION").to_string(),
        fqdn: state.settings.general.public_url.to_string(),
        oidc: state.settings.oidc.clone(),
    })
}
