use ginepro::LoadBalancedChannel;
use plugin_proto::{
    info_provider_service_client::InfoProviderServiceClient,
    link_resolver_service_client::LinkResolverServiceClient,
    search_service_client::SearchServiceClient,
    source_provider_service_client::SourceProviderServiceClient,
};

pub struct Clients {
    pub info: InfoProviderServiceClient<LoadBalancedChannel>,
    pub search: SearchServiceClient<LoadBalancedChannel>,
    pub source_provider: SourceProviderServiceClient<LoadBalancedChannel>,
    pub link_resolver: LinkResolverServiceClient<LoadBalancedChannel>,
}

impl Clients {
    pub fn from_channel(channel: LoadBalancedChannel) -> Self {
        let info = InfoProviderServiceClient::new(channel.clone());
        let search = SearchServiceClient::new(channel.clone());
        let source_provider = SourceProviderServiceClient::new(channel.clone());
        let link_resolver = LinkResolverServiceClient::new(channel);

        Self {
            info,
            search,
            source_provider,
            link_resolver,
        }
    }
}
