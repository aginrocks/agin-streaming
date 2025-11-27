use dashmap::{DashMap, DashSet};

use crate::plugin::Plugin;

#[derive(Default)]
pub struct PluginHost {
    plugins: DashMap<(String, u16), Plugin>,
    providers: DashMap<String, DashSet<(String, u16)>>,
}

impl PluginHost {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn register_plugin(&self, plugin: Plugin) {
        self.plugins
            .insert((plugin.hostname.clone(), plugin.port), plugin);
    }
}
