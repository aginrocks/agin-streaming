mod info;

use plugin_sdk::sdk::PluginSdk;

use crate::info::info;

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

    let sdk = PluginSdk::builder(state).add_service(info());

    // let sdk = PluginSdk::builder().add_service(InfoProviderServer::new(greeter));

    // Server::builder()
    //     .add_service(InfoProviderServer::new(greeter))
    //     .serve(addr)
    //     .await?;

    Ok(())
}
