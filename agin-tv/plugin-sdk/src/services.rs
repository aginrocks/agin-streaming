mod info;
mod link_resolver;
mod search;
mod source_provider;

use std::{net::SocketAddr, sync::Arc};

use plugin_proto::{
    info_provider_service_server::InfoProviderServiceServer,
    link_resolver_service_server::LinkResolverServiceServer,
    search_service_server::SearchServiceServer,
    source_provider_service_server::SourceProviderServiceServer,
};
use tonic::transport::Server;

use crate::{
    sdk::PluginSdk,
    services::{
        info::InfoProvider, link_resolver::LinkResolver, search::Search,
        source_provider::SourceProvider,
    },
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
    let source_provider = SourceProvider { sdk: sdk.clone() };
    let link_resolver = LinkResolver { sdk: sdk.clone() };

    Server::builder()
        .add_service(InfoProviderServiceServer::new(info))
        .add_service(SearchServiceServer::new(search))
        .add_service(SourceProviderServiceServer::new(source_provider))
        .add_service(LinkResolverServiceServer::new(link_resolver))
        .serve(address)
        .await?;

    Ok(())
}
