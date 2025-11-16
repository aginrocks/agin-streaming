mod info;

use plugin_sdk::sdk::PluginSdk;
use tonic::transport::Server;

use crate::{
    info::Info,
    plugin::{PluginInfo, info_provider_server::InfoProviderServer},
};

pub mod plugin {
    tonic::include_proto!("plugin");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let addr = "[::1]:50051".parse()?;
    let greeter = Info::default();

    let sdk = PluginSdk::builder().add_service(InfoProviderServer::new(greeter));

    // Server::builder()
    //     .add_service(InfoProviderServer::new(greeter))
    //     .serve(addr)
    //     .await?;

    Ok(())
}
