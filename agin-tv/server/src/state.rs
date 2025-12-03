use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{plugins::PluginsConfig, settings::Settings};

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub db: DatabaseConnection,
    pub tmdb: Arc<tmdb_api::client::ReqwestClient>,
    pub plugins: Arc<PluginsConfig>,
}
