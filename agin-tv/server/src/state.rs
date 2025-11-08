use std::sync::Arc;

use crate::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
}
