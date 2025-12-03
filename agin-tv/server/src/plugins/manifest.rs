use std::collections::HashMap;

use color_eyre::{
    Result,
    eyre::{Context, ContextCompat},
};
use serde::Deserialize;

use crate::plugins::PluginOptions;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PluginEntry {
    Short(String),
    Detailed {
        address: String,
        load_balance: Option<bool>,
    },
}

impl PluginEntry {
    pub fn address(&self) -> &String {
        match self {
            PluginEntry::Short(address) => address,
            PluginEntry::Detailed { address, .. } => address,
        }
    }
}

impl PluginOptions {
    pub fn try_from_entry(name: String, entry: PluginEntry) -> Result<Self> {
        let address = entry.address().clone();
        let (host, port) = address
            .split_once(':')
            .wrap_err("Address must be in format host:port")?;
        let port = port.parse::<u16>().wrap_err("Port must be valid")?;

        match entry {
            PluginEntry::Short(_) => Ok(PluginOptions {
                load_balance: true,
                host: host.into(),
                port,
                name,
            }),
            PluginEntry::Detailed { load_balance, .. } => Ok(PluginOptions {
                load_balance: load_balance.unwrap_or(true),
                host: host.into(),
                port,
                name,
            }),
        }
    }
}

#[derive(Default, Deserialize, Debug)]
pub struct PluginsManifest {
    pub plugins: HashMap<String, PluginEntry>,
}
