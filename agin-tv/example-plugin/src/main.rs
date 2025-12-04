mod info;

use plugin_sdk::sdk::PluginSdk;

use crate::info::info;

#[derive(Clone)]
pub struct State {
    pub a: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let addr = "[::1]:50051".parse()?;
    // let greeter = Info::default();
    //
    //

    let state = State { a: "a".into() };

    let sdk = PluginSdk::builder(state).add_services(vec![info()]);

    sdk.listen("0.0.0.0:50051".parse().unwrap()).await?;

    // let sdk = PluginSdk::builder().add_service(InfoProviderServer::new(greeter));

    // Server::builder()
    //     .add_service(InfoProviderServer::new(greeter))
    //     .serve(addr)
    //     .await?;

    Ok(())
}
