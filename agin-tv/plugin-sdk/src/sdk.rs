use std::sync::Arc;

use crate::{
    handler::{HandlerInfo, HandlerMetadata},
    plugin::Service,
    services,
};

#[derive(Default)]
pub struct PluginSdk<S: Send + Sync + Clone + 'static> {
    pub manifests: Vec<Service>,
    services: Vec<HandlerMetadata<S>>,
    state: S,
}

pub trait HasInner {
    type Inner;
    fn inner(&self) -> &Self::Inner;
}

impl<S: Send + Sync + Clone + 'static> PluginSdk<S> {
    pub fn builder(state: S) -> Self {
        Self {
            manifests: Vec::new(),
            services: Vec::new(),
            state,
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

    pub async fn listen(self) -> Result<(), tonic::transport::Error> {
        services::serve(self, "[::1]:50051".parse().unwrap()).await
    }

    // pub fn add_service<S>(mut self, service: S) -> Self
    // where
    //     S: Service<Request<Body>, Error = Infallible>
    //         + NamedService
    //         + Clone
    //         + Send
    //         + Sync
    //         + AginService
    //         + 'static,
    //     S::Response: axum::response::IntoResponse,
    //     S::Future: Send + 'static,
    // {
    //     let service_details = service.metadata();
    //     self.services.push(service_details);
    //     self.server.add_service(service);

    //     self
    // }

    // pub fn start(self, addr: std::net::SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    //     let mut server = self.server;

    //     for service in self.services {
    //         let svc = service.metadata();
    //         server = server.add_service(svc);
    //     }

    //     server.serve(addr).await?;

    //     Ok(())
    // }
}
