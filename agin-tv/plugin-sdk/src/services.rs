mod info;

use std::{net::SocketAddr, sync::Arc};

use tonic::transport::Server;

use crate::{
    plugin::info_provider_service_server::InfoProviderServiceServer, sdk::PluginSdk,
    services::info::InfoProvider,
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

    Server::builder()
        .add_service(InfoProviderServiceServer::new(info))
        .serve(address)
        .await?;

    Ok(())
}
