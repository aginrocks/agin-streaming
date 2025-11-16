use std::convert::Infallible;

use axum::http::Request;
use tonic::{body::Body, server::NamedService, transport::Server};
use tower_service::Service;

use crate::service::AginService;

#[derive(Default)]
pub struct PluginSdk {
    server: Server,
    services: Vec<crate::plugin::Service>,
}

pub trait HasInner {
    type Inner;
    fn inner(&self) -> &Self::Inner;
}

// impl<S> AginService for S
// where
//     S: tonic::server::NamedService + Clone + Send + Sync + HasInner + 'static,
// {
//     fn metadata(&self) -> crate::plugin::Service {
//         todo!()
//     }
// }

impl PluginSdk {
    pub fn builder() -> Self {
        Self {
            server: Server::builder(),
            services: Vec::new(),
        }
    }

    pub fn add_service<S>(mut self, service: S) -> Self
    where
        S: Service<Request<Body>, Error = Infallible>
            + NamedService
            + Clone
            + Send
            + Sync
            + AginService
            + 'static,
        S::Response: axum::response::IntoResponse,
        S::Future: Send + 'static,
    {
        let service_details = service.metadata();
        self.services.push(service_details);
        self.server.add_service(service);

        self
    }

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
