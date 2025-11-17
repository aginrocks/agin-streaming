use crate::api::cache::CacheManager;

pub mod cache;

pub struct PluginApi {
    pub cache: CacheManager,
}
