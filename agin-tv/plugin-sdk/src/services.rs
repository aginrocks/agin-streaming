mod info;
mod search;

use std::{net::SocketAddr, sync::Arc};

use tonic::transport::Server;

use crate::{
    plugin::{
        info_provider_service_server::InfoProviderServiceServer,
        search_service_server::SearchServiceServer,
    },
    sdk::PluginSdk,
    services::{info::InfoProvider, search::Search},
};

/// Serves gRPC endpoints. Accepts a Plugin SDK isntance, which should be initialized with all services.
///
/// # Generics
///
/// - `S`: Shared state type across the plugin lifecycle.
pub async fn serve<S: Send + Sync + Clone + 'static>(
    sdk: PluginSdk<S>,
    address: SocketAddr,
) -> Result<(), tonic::transport::Error> {
    let sdk = Arc::new(sdk);

    let info = InfoProvider { sdk: sdk.clone() };
    let search = Search { sdk: sdk.clone() };

    Server::builder()
        .add_service(InfoProviderServiceServer::new(info))
        .add_service(SearchServiceServer::new(search))
        .serve(address)
        .await?;

    Ok(())
}
