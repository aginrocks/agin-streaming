use std::net::SocketAddr;

use tonic::transport::Server;

use crate::sdk::PluginSdk;

/// Serves gRPC endpoints. Accepts a Plugin SDK isntance, which should be initialized with all services.
///
/// # Generics
///
/// - `S`: Shared state type across the plugin lifecycle.
pub async fn serve<S: Send + Sync + 'static>(
    sdk: &PluginSdk<S>,
    address: SocketAddr,
) -> Result<(), tonic::transport::Error> {
    // let server = Server::builder()
    // .add_service(todo!())
    // .serve(address)
    // .await?;

    Ok(())
}
