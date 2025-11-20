use std::{convert::Infallible, sync::Arc};

use axum::http::Request;
use tonic::{body::Body, server::NamedService, transport::Server};
use tower_service::Service;

use crate::service::ServiceInfo;

#[derive(Default)]
pub struct PluginSdk<S: Send + Sync + 'static> {
    services: Vec<ServiceInfo<S>>,
    state: Arc<S>,
}

pub trait HasInner {
    type Inner;
    fn inner(&self) -> &Self::Inner;
}

impl<S: Send + Sync + 'static> PluginSdk<S> {
    pub fn builder(state: S) -> Self {
        Self {
            services: Vec::new(),
            state: Arc::new(state),
        }
    }

    pub fn add_service(mut self, service: ServiceInfo<S>) -> Self {
        self.services.push(service);
        self
    }

    pub fn add_services(mut self, services: Vec<ServiceInfo<S>>) -> Self {
        self.services.extend(services);
        self
    }

    pub async fn listen(self) {}

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
