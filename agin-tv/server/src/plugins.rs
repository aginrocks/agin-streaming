mod manifest;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use color_eyre::{Result, eyre::eyre};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher, event::ModifyKind};
use plugin_host::{PluginHost, plugin::Plugin};
use serde::Deserialize;
use tokio::{fs, sync::mpsc};
use tracing::{error, info};

use crate::plugins::manifest::{PluginEntry, PluginsManifest};

#[derive(Debug)]
pub struct PluginOptions {
    host: String,
    port: u16,
    load_balance: bool,
    name: String,
}

/// Reads the configuration file and loads plugins into `PluginHost`
///
/// Only one instance should be watching at any given time to avoid race conditions.
#[derive(Default)]
pub struct PluginsConfig {
    pub host: Arc<PluginHost>,
    pub path: PathBuf,
}

impl PluginsConfig {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            ..Default::default()
        }
    }

    pub async fn watch(&self) -> Result<()> {
        let (tx, mut rx) = mpsc::channel(8);

        let mut watcher = RecommendedWatcher::new(
            move |event| {
                if let Err(e) = tx.try_send(event) {
                    error!("Watch channel send error: {:?}", e);
                }
            },
            notify::Config::default(),
        )?;

        watcher.watch(&self.path, RecursiveMode::NonRecursive)?;

        while let Some(event) = rx.recv().await {
            match event {
                Ok(event) => match event.kind {
                    EventKind::Modify(ModifyKind::Data(_)) => {
                        info!("Reloading plugins config, reason: plugins config modified");
                        if let Err(e) = self.load().await {
                            error!("Failed to reload plugins config: {e:?}");
                        }
                    }
                    EventKind::Remove(_) => {
                        error!("Plugins config file removed");
                    }
                    _ => {}
                },
                Err(e) => error!("Watch error: {:?}", e),
            }
        }

        Ok(())
    }

    async fn read_manifest(&self) -> Result<PluginsManifest> {
        let content = fs::read_to_string(&self.path).await?;
        let manifest = toml::from_str(&content)?;
        Ok(manifest)
    }

    async fn load(&self) -> Result<()> {
        let manifest = self.read_manifest().await?;

        for (name, entry) in manifest.plugins {
            if let Err(e) = self.load_entry(name.clone(), entry).await {
                error!("Failed to load plugin {name}: {e:?}");
            }
        }

        Ok(())
    }

    async fn load_entry(&self, name: String, entry: PluginEntry) -> Result<()> {
        let options = PluginOptions::try_from_entry(name, entry.clone())?;

        if self.host.has_plugin(&options.host, options.port) {
            info!(
                "Plugin {}:{} already loaded, skipping",
                options.host, options.port
            );
            return Ok(());
        }

        let plugin = Plugin::new_connect(options.host, options.port)
            .await
            .map_err(|_| eyre!("Failed to connect to the plugin"))?;

        self.host.register_plugin(plugin).await;

        Ok(())
    }
}
