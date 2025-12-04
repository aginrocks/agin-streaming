use plugin_proto::Service;
use std::{net::SocketAddr, sync::Arc};

use crate::{cache::CacheManager, context::Context, handler::HandlerMetadata, services};

pub struct PluginSdk<S: Send + Sync + Clone + 'static> {
    pub manifests: Vec<Service>,
    pub(crate) services: Vec<HandlerMetadata<S>>,
    pub context: Context<S>,
}

pub trait HasInner {
    type Inner;
    fn inner(&self) -> &Self::Inner;
}

impl<S: Send + Sync + Clone + 'static> PluginSdk<S> {
    pub fn builder(state: S) -> Self {
        let context = Context {
            state: Arc::new(state.clone()),
            cache: Arc::new(CacheManager::new()),
        };

        Self {
            manifests: Vec::new(),
            services: Vec::new(),
            context,
        }
    }

    pub fn add_services(mut self, services: Vec<HandlerMetadata<S>>) -> Self {
        let manifests = services
            .iter()
            .cloned()
            .map(Service::from)
            .collect::<Vec<Service>>();
        self.manifests.extend(manifests);
        self.services.extend(services);
        self
    }

    pub async fn listen(self, address: SocketAddr) -> Result<(), tonic::transport::Error> {
        services::serve(self, address).await
    }
}
