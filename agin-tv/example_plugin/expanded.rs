#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
mod info {
    use plugin_sdk::service::AginService;
    use tonic::{Request, Response, Status};
    use crate::plugin::{
        PluginInfo, info_provider_server::{InfoProvider, InfoProviderServer},
    };
    pub struct Info {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Info {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "Info")
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Info {
        #[inline]
        fn default() -> Info {
            Info {}
        }
    }
    impl AginService for Info {
        fn metadata(&self) -> plugin_sdk::plugin::Service {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl InfoProvider for Info {
        #[allow(
            elided_named_lifetimes,
            clippy::async_yields_async,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::needless_arbitrary_self_type,
            clippy::no_effect_underscore_binding,
            clippy::shadow_same,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn get_plugin_info<'life0, 'async_trait>(
            &'life0 self,
            request: Request<()>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = Result<Response<PluginInfo>, Status>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    Result<Response<PluginInfo>, Status>,
                > {
                    #[allow(unreachable_code)] return __ret;
                }
                let __self = self;
                let request = request;
                let __ret: Result<Response<PluginInfo>, Status> = {
                    {
                        ::std::io::_print(format_args!("Got a request\n"));
                    };
                    let reply = PluginInfo {
                        id: "1".into(),
                        display_name: "a".into(),
                        version: "0.1.0".into(),
                        services: ::alloc::vec::Vec::new(),
                    };
                    Ok(Response::new(reply))
                };
                #[allow(unreachable_code)] __ret
            })
        }
    }
}
use plugin_sdk::sdk::PluginSdk;
use tonic::transport::Server;
use crate::{info::Info, plugin::{PluginInfo, info_provider_server::InfoProviderServer}};
pub mod plugin {
    /// Information about a plugin
    pub struct PluginInfo {
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub display_name: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub version: ::prost::alloc::string::String,
        /// Each plugin exposes a list of services
        #[prost(message, repeated, tag = "4")]
        pub services: ::prost::alloc::vec::Vec<Service>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PluginInfo {
        #[inline]
        fn clone(&self) -> PluginInfo {
            PluginInfo {
                id: ::core::clone::Clone::clone(&self.id),
                display_name: ::core::clone::Clone::clone(&self.display_name),
                version: ::core::clone::Clone::clone(&self.version),
                services: ::core::clone::Clone::clone(&self.services),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PluginInfo {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PluginInfo {
        #[inline]
        fn eq(&self, other: &PluginInfo) -> bool {
            self.id == other.id && self.display_name == other.display_name
                && self.version == other.version && self.services == other.services
        }
    }
    impl ::prost::Message for PluginInfo {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.id != "" {
                ::prost::encoding::string::encode(1u32, &self.id, buf);
            }
            if self.display_name != "" {
                ::prost::encoding::string::encode(2u32, &self.display_name, buf);
            }
            if self.version != "" {
                ::prost::encoding::string::encode(3u32, &self.version, buf);
            }
            for msg in &self.services {
                ::prost::encoding::message::encode(4u32, msg, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "PluginInfo";
            match tag {
                1u32 => {
                    let mut value = &mut self.id;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "id");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.display_name;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "display_name");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.version;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "version");
                            error
                        })
                }
                4u32 => {
                    let mut value = &mut self.services;
                    ::prost::encoding::message::merge_repeated(
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "services");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.id != "" {
                    ::prost::encoding::string::encoded_len(1u32, &self.id)
                } else {
                    0
                }
                + if self.display_name != "" {
                    ::prost::encoding::string::encoded_len(2u32, &self.display_name)
                } else {
                    0
                }
                + if self.version != "" {
                    ::prost::encoding::string::encoded_len(3u32, &self.version)
                } else {
                    0
                }
                + ::prost::encoding::message::encoded_len_repeated(4u32, &self.services)
        }
        fn clear(&mut self) {
            self.id.clear();
            self.display_name.clear();
            self.version.clear();
            self.services.clear();
        }
    }
    impl ::core::default::Default for PluginInfo {
        fn default() -> Self {
            PluginInfo {
                id: ::prost::alloc::string::String::new(),
                display_name: ::prost::alloc::string::String::new(),
                version: ::prost::alloc::string::String::new(),
                services: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for PluginInfo {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("PluginInfo");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.id)
                };
                builder.field("id", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.display_name)
                };
                builder.field("display_name", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.version)
                };
                builder.field("version", &wrapper)
            };
            let builder = {
                let wrapper = &self.services;
                builder.field("services", &wrapper)
            };
            builder.finish()
        }
    }
    /// Describes one service implemented by this plugin
    pub struct Service {
        #[prost(enumeration = "ServiceType", tag = "1")]
        pub r#type: i32,
        /// The providers (e.g. site IDs) that this service supports
        /// e.g. \["siteA", "siteB"\]
        #[prost(string, repeated, tag = "2")]
        pub providers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Service {
        #[inline]
        fn clone(&self) -> Service {
            Service {
                r#type: ::core::clone::Clone::clone(&self.r#type),
                providers: ::core::clone::Clone::clone(&self.providers),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Service {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Service {
        #[inline]
        fn eq(&self, other: &Service) -> bool {
            self.r#type == other.r#type && self.providers == other.providers
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Service {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<i32>;
            let _: ::core::cmp::AssertParamIsEq<
                ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Service {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.r#type, state);
            ::core::hash::Hash::hash(&self.providers, state)
        }
    }
    impl ::prost::Message for Service {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.r#type != ServiceType::default() as i32 {
                ::prost::encoding::int32::encode(1u32, &self.r#type, buf);
            }
            ::prost::encoding::string::encode_repeated(2u32, &self.providers, buf);
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "Service";
            match tag {
                1u32 => {
                    let mut value = &mut self.r#type;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "r#type");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.providers;
                    ::prost::encoding::string::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "providers");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.r#type != ServiceType::default() as i32 {
                    ::prost::encoding::int32::encoded_len(1u32, &self.r#type)
                } else {
                    0
                }
                + ::prost::encoding::string::encoded_len_repeated(2u32, &self.providers)
        }
        fn clear(&mut self) {
            self.r#type = ServiceType::default() as i32;
            self.providers.clear();
        }
    }
    impl ::core::default::Default for Service {
        fn default() -> Self {
            Service {
                r#type: ServiceType::default() as i32,
                providers: ::prost::alloc::vec::Vec::new(),
            }
        }
    }
    impl ::core::fmt::Debug for Service {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Service");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a i32);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let res: ::core::result::Result<ServiceType, _> = ::core::convert::TryFrom::try_from(
                                *self.0,
                            );
                            match res {
                                Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                            }
                        }
                    }
                    ScalarWrapper(&self.r#type)
                };
                builder.field("r#type", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.providers)
                };
                builder.field("providers", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl Service {
        ///Returns the enum value of `type`, or the default if the field is set to an invalid enum value.
        pub fn r#type(&self) -> ServiceType {
            ::core::convert::TryFrom::try_from(self.r#type)
                .unwrap_or(ServiceType::default())
        }
        ///Sets `type` to the provided enum value.
        pub fn set_type(&mut self, value: ServiceType) {
            self.r#type = value as i32;
        }
    }
    pub struct SearchRequest {
        #[prost(enumeration = "ContentType", tag = "1")]
        pub r#type: i32,
        #[prost(int32, tag = "2")]
        pub tmdb_id: i32,
        #[prost(string, tag = "3")]
        pub title: ::prost::alloc::string::String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SearchRequest {
        #[inline]
        fn clone(&self) -> SearchRequest {
            SearchRequest {
                r#type: ::core::clone::Clone::clone(&self.r#type),
                tmdb_id: ::core::clone::Clone::clone(&self.tmdb_id),
                title: ::core::clone::Clone::clone(&self.title),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SearchRequest {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SearchRequest {
        #[inline]
        fn eq(&self, other: &SearchRequest) -> bool {
            self.r#type == other.r#type && self.tmdb_id == other.tmdb_id
                && self.title == other.title
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for SearchRequest {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<i32>;
            let _: ::core::cmp::AssertParamIsEq<::prost::alloc::string::String>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for SearchRequest {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.r#type, state);
            ::core::hash::Hash::hash(&self.tmdb_id, state);
            ::core::hash::Hash::hash(&self.title, state)
        }
    }
    impl ::prost::Message for SearchRequest {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.r#type != ContentType::default() as i32 {
                ::prost::encoding::int32::encode(1u32, &self.r#type, buf);
            }
            if self.tmdb_id != 0i32 {
                ::prost::encoding::int32::encode(2u32, &self.tmdb_id, buf);
            }
            if self.title != "" {
                ::prost::encoding::string::encode(3u32, &self.title, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "SearchRequest";
            match tag {
                1u32 => {
                    let mut value = &mut self.r#type;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "r#type");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.tmdb_id;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "tmdb_id");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.title;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "title");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.r#type != ContentType::default() as i32 {
                    ::prost::encoding::int32::encoded_len(1u32, &self.r#type)
                } else {
                    0
                }
                + if self.tmdb_id != 0i32 {
                    ::prost::encoding::int32::encoded_len(2u32, &self.tmdb_id)
                } else {
                    0
                }
                + if self.title != "" {
                    ::prost::encoding::string::encoded_len(3u32, &self.title)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.r#type = ContentType::default() as i32;
            self.tmdb_id = 0i32;
            self.title.clear();
        }
    }
    impl ::core::default::Default for SearchRequest {
        fn default() -> Self {
            SearchRequest {
                r#type: ContentType::default() as i32,
                tmdb_id: 0i32,
                title: ::prost::alloc::string::String::new(),
            }
        }
    }
    impl ::core::fmt::Debug for SearchRequest {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("SearchRequest");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a i32);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let res: ::core::result::Result<ContentType, _> = ::core::convert::TryFrom::try_from(
                                *self.0,
                            );
                            match res {
                                Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                            }
                        }
                    }
                    ScalarWrapper(&self.r#type)
                };
                builder.field("r#type", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.tmdb_id)
                };
                builder.field("tmdb_id", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.title)
                };
                builder.field("title", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl SearchRequest {
        ///Returns the enum value of `type`, or the default if the field is set to an invalid enum value.
        pub fn r#type(&self) -> ContentType {
            ::core::convert::TryFrom::try_from(self.r#type)
                .unwrap_or(ContentType::default())
        }
        ///Sets `type` to the provided enum value.
        pub fn set_type(&mut self, value: ContentType) {
            self.r#type = value as i32;
        }
    }
    pub struct SearchResponses {
        #[prost(message, repeated, tag = "1")]
        pub results: ::prost::alloc::vec::Vec<SearchResult>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SearchResponses {
        #[inline]
        fn clone(&self) -> SearchResponses {
            SearchResponses {
                results: ::core::clone::Clone::clone(&self.results),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SearchResponses {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SearchResponses {
        #[inline]
        fn eq(&self, other: &SearchResponses) -> bool {
            self.results == other.results
        }
    }
    impl ::prost::Message for SearchResponses {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            for msg in &self.results {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "SearchResponses";
            match tag {
                1u32 => {
                    let mut value = &mut self.results;
                    ::prost::encoding::message::merge_repeated(
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "results");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::prost::encoding::message::encoded_len_repeated(1u32, &self.results)
        }
        fn clear(&mut self) {
            self.results.clear();
        }
    }
    impl ::core::default::Default for SearchResponses {
        fn default() -> Self {
            SearchResponses {
                results: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for SearchResponses {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("SearchResponses");
            let builder = {
                let wrapper = &self.results;
                builder.field("results", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct SearchResult {
        /// Provider that can handle this match
        #[prost(string, tag = "1")]
        pub provider_id: ::prost::alloc::string::String,
        /// Opaque plugin-specific identifier that uniquely identifies content on that provider
        #[prost(string, tag = "2")]
        pub r#match: ::prost::alloc::string::String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SearchResult {
        #[inline]
        fn clone(&self) -> SearchResult {
            SearchResult {
                provider_id: ::core::clone::Clone::clone(&self.provider_id),
                r#match: ::core::clone::Clone::clone(&self.r#match),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SearchResult {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SearchResult {
        #[inline]
        fn eq(&self, other: &SearchResult) -> bool {
            self.provider_id == other.provider_id && self.r#match == other.r#match
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for SearchResult {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::prost::alloc::string::String>;
            let _: ::core::cmp::AssertParamIsEq<::prost::alloc::string::String>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for SearchResult {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.provider_id, state);
            ::core::hash::Hash::hash(&self.r#match, state)
        }
    }
    impl ::prost::Message for SearchResult {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.provider_id != "" {
                ::prost::encoding::string::encode(1u32, &self.provider_id, buf);
            }
            if self.r#match != "" {
                ::prost::encoding::string::encode(2u32, &self.r#match, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "SearchResult";
            match tag {
                1u32 => {
                    let mut value = &mut self.provider_id;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "provider_id");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.r#match;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "r#match");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.provider_id != "" {
                    ::prost::encoding::string::encoded_len(1u32, &self.provider_id)
                } else {
                    0
                }
                + if self.r#match != "" {
                    ::prost::encoding::string::encoded_len(2u32, &self.r#match)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.provider_id.clear();
            self.r#match.clear();
        }
    }
    impl ::core::default::Default for SearchResult {
        fn default() -> Self {
            SearchResult {
                provider_id: ::prost::alloc::string::String::new(),
                r#match: ::prost::alloc::string::String::new(),
            }
        }
    }
    impl ::core::fmt::Debug for SearchResult {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("SearchResult");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.provider_id)
                };
                builder.field("provider_id", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.r#match)
                };
                builder.field("r#match", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct SourceRequest {
        #[prost(enumeration = "ContentType", tag = "1")]
        pub r#type: i32,
        /// provider that this request is targeting (e.g. 'siteA')
        #[prost(string, tag = "2")]
        pub provider_id: ::prost::alloc::string::String,
        #[prost(int32, tag = "3")]
        pub tmdb_id: i32,
        /// Optional search match; preferred over TMDB ID
        #[prost(string, optional, tag = "4")]
        pub r#match: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "5")]
        pub season: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "6")]
        pub episode: ::core::option::Option<i32>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SourceRequest {
        #[inline]
        fn clone(&self) -> SourceRequest {
            SourceRequest {
                r#type: ::core::clone::Clone::clone(&self.r#type),
                provider_id: ::core::clone::Clone::clone(&self.provider_id),
                tmdb_id: ::core::clone::Clone::clone(&self.tmdb_id),
                r#match: ::core::clone::Clone::clone(&self.r#match),
                season: ::core::clone::Clone::clone(&self.season),
                episode: ::core::clone::Clone::clone(&self.episode),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SourceRequest {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SourceRequest {
        #[inline]
        fn eq(&self, other: &SourceRequest) -> bool {
            self.r#type == other.r#type && self.tmdb_id == other.tmdb_id
                && self.provider_id == other.provider_id && self.r#match == other.r#match
                && self.season == other.season && self.episode == other.episode
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for SourceRequest {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<i32>;
            let _: ::core::cmp::AssertParamIsEq<::prost::alloc::string::String>;
            let _: ::core::cmp::AssertParamIsEq<
                ::core::option::Option<::prost::alloc::string::String>,
            >;
            let _: ::core::cmp::AssertParamIsEq<::core::option::Option<i32>>;
            let _: ::core::cmp::AssertParamIsEq<::core::option::Option<i32>>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for SourceRequest {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.r#type, state);
            ::core::hash::Hash::hash(&self.provider_id, state);
            ::core::hash::Hash::hash(&self.tmdb_id, state);
            ::core::hash::Hash::hash(&self.r#match, state);
            ::core::hash::Hash::hash(&self.season, state);
            ::core::hash::Hash::hash(&self.episode, state)
        }
    }
    impl ::prost::Message for SourceRequest {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.r#type != ContentType::default() as i32 {
                ::prost::encoding::int32::encode(1u32, &self.r#type, buf);
            }
            if self.provider_id != "" {
                ::prost::encoding::string::encode(2u32, &self.provider_id, buf);
            }
            if self.tmdb_id != 0i32 {
                ::prost::encoding::int32::encode(3u32, &self.tmdb_id, buf);
            }
            if let ::core::option::Option::Some(ref value) = self.r#match {
                ::prost::encoding::string::encode(4u32, value, buf);
            }
            if let ::core::option::Option::Some(ref value) = self.season {
                ::prost::encoding::int32::encode(5u32, value, buf);
            }
            if let ::core::option::Option::Some(ref value) = self.episode {
                ::prost::encoding::int32::encode(6u32, value, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "SourceRequest";
            match tag {
                1u32 => {
                    let mut value = &mut self.r#type;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "r#type");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.provider_id;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "provider_id");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.tmdb_id;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "tmdb_id");
                            error
                        })
                }
                4u32 => {
                    let mut value = &mut self.r#match;
                    ::prost::encoding::string::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "r#match");
                            error
                        })
                }
                5u32 => {
                    let mut value = &mut self.season;
                    ::prost::encoding::int32::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "season");
                            error
                        })
                }
                6u32 => {
                    let mut value = &mut self.episode;
                    ::prost::encoding::int32::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "episode");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.r#type != ContentType::default() as i32 {
                    ::prost::encoding::int32::encoded_len(1u32, &self.r#type)
                } else {
                    0
                }
                + if self.provider_id != "" {
                    ::prost::encoding::string::encoded_len(2u32, &self.provider_id)
                } else {
                    0
                }
                + if self.tmdb_id != 0i32 {
                    ::prost::encoding::int32::encoded_len(3u32, &self.tmdb_id)
                } else {
                    0
                }
                + self
                    .r#match
                    .as_ref()
                    .map_or(
                        0,
                        |value| ::prost::encoding::string::encoded_len(4u32, value),
                    )
                + self
                    .season
                    .as_ref()
                    .map_or(
                        0,
                        |value| ::prost::encoding::int32::encoded_len(5u32, value),
                    )
                + self
                    .episode
                    .as_ref()
                    .map_or(
                        0,
                        |value| ::prost::encoding::int32::encoded_len(6u32, value),
                    )
        }
        fn clear(&mut self) {
            self.r#type = ContentType::default() as i32;
            self.provider_id.clear();
            self.tmdb_id = 0i32;
            self.r#match = ::core::option::Option::None;
            self.season = ::core::option::Option::None;
            self.episode = ::core::option::Option::None;
        }
    }
    impl ::core::default::Default for SourceRequest {
        fn default() -> Self {
            SourceRequest {
                r#type: ContentType::default() as i32,
                provider_id: ::prost::alloc::string::String::new(),
                tmdb_id: 0i32,
                r#match: ::core::option::Option::None,
                season: ::core::option::Option::None,
                episode: ::core::option::Option::None,
            }
        }
    }
    impl ::core::fmt::Debug for SourceRequest {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("SourceRequest");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a i32);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let res: ::core::result::Result<ContentType, _> = ::core::convert::TryFrom::try_from(
                                *self.0,
                            );
                            match res {
                                Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                            }
                        }
                    }
                    ScalarWrapper(&self.r#type)
                };
                builder.field("r#type", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.provider_id)
                };
                builder.field("provider_id", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.tmdb_id)
                };
                builder.field("tmdb_id", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::core::option::Option<::prost::alloc::string::String>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.r#match)
                };
                builder.field("r#match", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.season)
                };
                builder.field("season", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.episode)
                };
                builder.field("episode", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl SourceRequest {
        ///Returns the enum value of `type`, or the default if the field is set to an invalid enum value.
        pub fn r#type(&self) -> ContentType {
            ::core::convert::TryFrom::try_from(self.r#type)
                .unwrap_or(ContentType::default())
        }
        ///Sets `type` to the provided enum value.
        pub fn set_type(&mut self, value: ContentType) {
            self.r#type = value as i32;
        }
        ///Returns the value of `match`, or the default value if `match` is unset.
        pub fn r#match(&self) -> &str {
            match self.r#match {
                ::core::option::Option::Some(ref val) => &val[..],
                ::core::option::Option::None => "",
            }
        }
        ///Returns the value of `season`, or the default value if `season` is unset.
        pub fn season(&self) -> i32 {
            match self.season {
                ::core::option::Option::Some(val) => val,
                ::core::option::Option::None => 0i32,
            }
        }
        ///Returns the value of `episode`, or the default value if `episode` is unset.
        pub fn episode(&self) -> i32 {
            match self.episode {
                ::core::option::Option::Some(val) => val,
                ::core::option::Option::None => 0i32,
            }
        }
    }
    pub struct SourceResponse {
        #[prost(message, repeated, tag = "1")]
        pub sources: ::prost::alloc::vec::Vec<Source>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SourceResponse {
        #[inline]
        fn clone(&self) -> SourceResponse {
            SourceResponse {
                sources: ::core::clone::Clone::clone(&self.sources),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SourceResponse {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SourceResponse {
        #[inline]
        fn eq(&self, other: &SourceResponse) -> bool {
            self.sources == other.sources
        }
    }
    impl ::prost::Message for SourceResponse {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            for msg in &self.sources {
                ::prost::encoding::message::encode(1u32, msg, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "SourceResponse";
            match tag {
                1u32 => {
                    let mut value = &mut self.sources;
                    ::prost::encoding::message::merge_repeated(
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "sources");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::prost::encoding::message::encoded_len_repeated(1u32, &self.sources)
        }
        fn clear(&mut self) {
            self.sources.clear();
        }
    }
    impl ::core::default::Default for SourceResponse {
        fn default() -> Self {
            SourceResponse {
                sources: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::fmt::Debug for SourceResponse {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("SourceResponse");
            let builder = {
                let wrapper = &self.sources;
                builder.field("sources", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct Source {
        /// The provider that produced this source
        #[prost(string, tag = "1")]
        pub provider_id: ::prost::alloc::string::String,
        /// Link or identifier to hand to a link resolver
        #[prost(string, tag = "2")]
        pub link: ::prost::alloc::string::String,
        /// Optional human readable metadata
        #[prost(string, tag = "3")]
        pub quality: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub audio: ::prost::alloc::string::String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Source {
        #[inline]
        fn clone(&self) -> Source {
            Source {
                provider_id: ::core::clone::Clone::clone(&self.provider_id),
                link: ::core::clone::Clone::clone(&self.link),
                quality: ::core::clone::Clone::clone(&self.quality),
                audio: ::core::clone::Clone::clone(&self.audio),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Source {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Source {
        #[inline]
        fn eq(&self, other: &Source) -> bool {
            self.provider_id == other.provider_id && self.link == other.link
                && self.quality == other.quality && self.audio == other.audio
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Source {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::prost::alloc::string::String>;
            let _: ::core::cmp::AssertParamIsEq<::prost::alloc::string::String>;
            let _: ::core::cmp::AssertParamIsEq<::prost::alloc::string::String>;
            let _: ::core::cmp::AssertParamIsEq<::prost::alloc::string::String>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Source {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.provider_id, state);
            ::core::hash::Hash::hash(&self.link, state);
            ::core::hash::Hash::hash(&self.quality, state);
            ::core::hash::Hash::hash(&self.audio, state)
        }
    }
    impl ::prost::Message for Source {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.provider_id != "" {
                ::prost::encoding::string::encode(1u32, &self.provider_id, buf);
            }
            if self.link != "" {
                ::prost::encoding::string::encode(2u32, &self.link, buf);
            }
            if self.quality != "" {
                ::prost::encoding::string::encode(3u32, &self.quality, buf);
            }
            if self.audio != "" {
                ::prost::encoding::string::encode(4u32, &self.audio, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "Source";
            match tag {
                1u32 => {
                    let mut value = &mut self.provider_id;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "provider_id");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.link;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "link");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut self.quality;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "quality");
                            error
                        })
                }
                4u32 => {
                    let mut value = &mut self.audio;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "audio");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.provider_id != "" {
                    ::prost::encoding::string::encoded_len(1u32, &self.provider_id)
                } else {
                    0
                }
                + if self.link != "" {
                    ::prost::encoding::string::encoded_len(2u32, &self.link)
                } else {
                    0
                }
                + if self.quality != "" {
                    ::prost::encoding::string::encoded_len(3u32, &self.quality)
                } else {
                    0
                }
                + if self.audio != "" {
                    ::prost::encoding::string::encoded_len(4u32, &self.audio)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.provider_id.clear();
            self.link.clear();
            self.quality.clear();
            self.audio.clear();
        }
    }
    impl ::core::default::Default for Source {
        fn default() -> Self {
            Source {
                provider_id: ::prost::alloc::string::String::new(),
                link: ::prost::alloc::string::String::new(),
                quality: ::prost::alloc::string::String::new(),
                audio: ::prost::alloc::string::String::new(),
            }
        }
    }
    impl ::core::fmt::Debug for Source {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Source");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.provider_id)
                };
                builder.field("provider_id", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.link)
                };
                builder.field("link", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.quality)
                };
                builder.field("quality", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.audio)
                };
                builder.field("audio", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct ResolveRequest {
        #[prost(string, tag = "1")]
        pub provider_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub link: ::prost::alloc::string::String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ResolveRequest {
        #[inline]
        fn clone(&self) -> ResolveRequest {
            ResolveRequest {
                provider_id: ::core::clone::Clone::clone(&self.provider_id),
                link: ::core::clone::Clone::clone(&self.link),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ResolveRequest {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ResolveRequest {
        #[inline]
        fn eq(&self, other: &ResolveRequest) -> bool {
            self.provider_id == other.provider_id && self.link == other.link
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ResolveRequest {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::prost::alloc::string::String>;
            let _: ::core::cmp::AssertParamIsEq<::prost::alloc::string::String>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ResolveRequest {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.provider_id, state);
            ::core::hash::Hash::hash(&self.link, state)
        }
    }
    impl ::prost::Message for ResolveRequest {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.provider_id != "" {
                ::prost::encoding::string::encode(1u32, &self.provider_id, buf);
            }
            if self.link != "" {
                ::prost::encoding::string::encode(2u32, &self.link, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "ResolveRequest";
            match tag {
                1u32 => {
                    let mut value = &mut self.provider_id;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "provider_id");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut self.link;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "link");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.provider_id != "" {
                    ::prost::encoding::string::encoded_len(1u32, &self.provider_id)
                } else {
                    0
                }
                + if self.link != "" {
                    ::prost::encoding::string::encoded_len(2u32, &self.link)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.provider_id.clear();
            self.link.clear();
        }
    }
    impl ::core::default::Default for ResolveRequest {
        fn default() -> Self {
            ResolveRequest {
                provider_id: ::prost::alloc::string::String::new(),
                link: ::prost::alloc::string::String::new(),
            }
        }
    }
    impl ::core::fmt::Debug for ResolveRequest {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("ResolveRequest");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.provider_id)
                };
                builder.field("provider_id", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.link)
                };
                builder.field("link", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct ResolveResponse {
        /// Final playback URL
        #[prost(string, tag = "1")]
        pub url: ::prost::alloc::string::String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ResolveResponse {
        #[inline]
        fn clone(&self) -> ResolveResponse {
            ResolveResponse {
                url: ::core::clone::Clone::clone(&self.url),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ResolveResponse {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ResolveResponse {
        #[inline]
        fn eq(&self, other: &ResolveResponse) -> bool {
            self.url == other.url
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ResolveResponse {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::prost::alloc::string::String>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ResolveResponse {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.url, state)
        }
    }
    impl ::prost::Message for ResolveResponse {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            if self.url != "" {
                ::prost::encoding::string::encode(1u32, &self.url, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "ResolveResponse";
            match tag {
                1u32 => {
                    let mut value = &mut self.url;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "url");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0
                + if self.url != "" {
                    ::prost::encoding::string::encoded_len(1u32, &self.url)
                } else {
                    0
                }
        }
        fn clear(&mut self) {
            self.url.clear();
        }
    }
    impl ::core::default::Default for ResolveResponse {
        fn default() -> Self {
            ResolveResponse {
                url: ::prost::alloc::string::String::new(),
            }
        }
    }
    impl ::core::fmt::Debug for ResolveResponse {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("ResolveResponse");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.url)
                };
                builder.field("url", &wrapper)
            };
            builder.finish()
        }
    }
    /// High-level types (movies, shows, etc.)
    #[repr(i32)]
    pub enum ContentType {
        Movie = 0,
        TvShow = 1,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ContentType {
        #[inline]
        fn clone(&self) -> ContentType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ContentType {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ContentType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ContentType::Movie => "Movie",
                    ContentType::TvShow => "TvShow",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ContentType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ContentType {
        #[inline]
        fn eq(&self, other: &ContentType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ContentType {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ContentType {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ContentType {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ContentType,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ContentType {
        #[inline]
        fn cmp(&self, other: &ContentType) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    impl ContentType {
        ///Returns `true` if `value` is a variant of `ContentType`.
        pub fn is_valid(value: i32) -> bool {
            match value {
                0 => true,
                1 => true,
                _ => false,
            }
        }
        #[deprecated = "Use the TryFrom<i32> implementation instead"]
        ///Converts an `i32` to a `ContentType`, or `None` if `value` is not a valid variant.
        pub fn from_i32(value: i32) -> ::core::option::Option<ContentType> {
            match value {
                0 => ::core::option::Option::Some(ContentType::Movie),
                1 => ::core::option::Option::Some(ContentType::TvShow),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl ::core::default::Default for ContentType {
        fn default() -> ContentType {
            ContentType::Movie
        }
    }
    impl ::core::convert::From<ContentType> for i32 {
        fn from(value: ContentType) -> i32 {
            value as i32
        }
    }
    impl ::core::convert::TryFrom<i32> for ContentType {
        type Error = ::prost::UnknownEnumValue;
        fn try_from(
            value: i32,
        ) -> ::core::result::Result<ContentType, ::prost::UnknownEnumValue> {
            match value {
                0 => ::core::result::Result::Ok(ContentType::Movie),
                1 => ::core::result::Result::Ok(ContentType::TvShow),
                _ => ::core::result::Result::Err(::prost::UnknownEnumValue(value)),
            }
        }
    }
    impl ContentType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Movie => "MOVIE",
                Self::TvShow => "TV_SHOW",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MOVIE" => Some(Self::Movie),
                "TV_SHOW" => Some(Self::TvShow),
                _ => None,
            }
        }
    }
    #[repr(i32)]
    pub enum ServiceType {
        Search = 0,
        SourceProvider = 1,
        LinkResolver = 2,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ServiceType {
        #[inline]
        fn clone(&self) -> ServiceType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ServiceType {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ServiceType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ServiceType::Search => "Search",
                    ServiceType::SourceProvider => "SourceProvider",
                    ServiceType::LinkResolver => "LinkResolver",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ServiceType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ServiceType {
        #[inline]
        fn eq(&self, other: &ServiceType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ServiceType {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ServiceType {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ServiceType {
        #[inline]
        fn partial_cmp(
            &self,
            other: &ServiceType,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ServiceType {
        #[inline]
        fn cmp(&self, other: &ServiceType) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    impl ServiceType {
        ///Returns `true` if `value` is a variant of `ServiceType`.
        pub fn is_valid(value: i32) -> bool {
            match value {
                0 => true,
                1 => true,
                2 => true,
                _ => false,
            }
        }
        #[deprecated = "Use the TryFrom<i32> implementation instead"]
        ///Converts an `i32` to a `ServiceType`, or `None` if `value` is not a valid variant.
        pub fn from_i32(value: i32) -> ::core::option::Option<ServiceType> {
            match value {
                0 => ::core::option::Option::Some(ServiceType::Search),
                1 => ::core::option::Option::Some(ServiceType::SourceProvider),
                2 => ::core::option::Option::Some(ServiceType::LinkResolver),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl ::core::default::Default for ServiceType {
        fn default() -> ServiceType {
            ServiceType::Search
        }
    }
    impl ::core::convert::From<ServiceType> for i32 {
        fn from(value: ServiceType) -> i32 {
            value as i32
        }
    }
    impl ::core::convert::TryFrom<i32> for ServiceType {
        type Error = ::prost::UnknownEnumValue;
        fn try_from(
            value: i32,
        ) -> ::core::result::Result<ServiceType, ::prost::UnknownEnumValue> {
            match value {
                0 => ::core::result::Result::Ok(ServiceType::Search),
                1 => ::core::result::Result::Ok(ServiceType::SourceProvider),
                2 => ::core::result::Result::Ok(ServiceType::LinkResolver),
                _ => ::core::result::Result::Err(::prost::UnknownEnumValue(value)),
            }
        }
    }
    impl ServiceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Search => "SEARCH",
                Self::SourceProvider => "SOURCE_PROVIDER",
                Self::LinkResolver => "LINK_RESOLVER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEARCH" => Some(Self::Search),
                "SOURCE_PROVIDER" => Some(Self::SourceProvider),
                "LINK_RESOLVER" => Some(Self::LinkResolver),
                _ => None,
            }
        }
    }
    /// Generated client implementations.
    pub mod info_provider_client {
        #![allow(
            unused_variables,
            dead_code,
            missing_docs,
            clippy::wildcard_imports,
            clippy::let_unit_value,
        )]
        use tonic::codegen::*;
        use tonic::codegen::http::Uri;
        pub struct InfoProviderClient<T> {
            inner: tonic::client::Grpc<T>,
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug> ::core::fmt::Debug for InfoProviderClient<T> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "InfoProviderClient",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl<T: ::core::clone::Clone> ::core::clone::Clone for InfoProviderClient<T> {
            #[inline]
            fn clone(&self) -> InfoProviderClient<T> {
                InfoProviderClient {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        impl InfoProviderClient<tonic::transport::Channel> {
            /// Attempt to create a new client by connecting to a given endpoint.
            pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
            where
                D: TryInto<tonic::transport::Endpoint>,
                D::Error: Into<StdError>,
            {
                let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
                Ok(Self::new(conn))
            }
        }
        impl<T> InfoProviderClient<T>
        where
            T: tonic::client::GrpcService<tonic::body::Body>,
            T::Error: Into<StdError>,
            T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
            <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
        {
            pub fn new(inner: T) -> Self {
                let inner = tonic::client::Grpc::new(inner);
                Self { inner }
            }
            pub fn with_origin(inner: T, origin: Uri) -> Self {
                let inner = tonic::client::Grpc::with_origin(inner, origin);
                Self { inner }
            }
            pub fn with_interceptor<F>(
                inner: T,
                interceptor: F,
            ) -> InfoProviderClient<InterceptedService<T, F>>
            where
                F: tonic::service::Interceptor,
                T::ResponseBody: Default,
                T: tonic::codegen::Service<
                    http::Request<tonic::body::Body>,
                    Response = http::Response<
                        <T as tonic::client::GrpcService<
                            tonic::body::Body,
                        >>::ResponseBody,
                    >,
                >,
                <T as tonic::codegen::Service<
                    http::Request<tonic::body::Body>,
                >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
            {
                InfoProviderClient::new(InterceptedService::new(inner, interceptor))
            }
            /// Compress requests with the given encoding.
            ///
            /// This requires the server to support it otherwise it might respond with an
            /// error.
            #[must_use]
            pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.inner = self.inner.send_compressed(encoding);
                self
            }
            /// Enable decompressing responses.
            #[must_use]
            pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.inner = self.inner.accept_compressed(encoding);
                self
            }
            /// Limits the maximum size of a decoded message.
            ///
            /// Default: `4MB`
            #[must_use]
            pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
                self.inner = self.inner.max_decoding_message_size(limit);
                self
            }
            /// Limits the maximum size of an encoded message.
            ///
            /// Default: `usize::MAX`
            #[must_use]
            pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
                self.inner = self.inner.max_encoding_message_size(limit);
                self
            }
            pub async fn get_plugin_info(
                &mut self,
                request: impl tonic::IntoRequest<()>,
            ) -> std::result::Result<tonic::Response<super::PluginInfo>, tonic::Status> {
                self.inner
                    .ready()
                    .await
                    .map_err(|e| {
                        tonic::Status::unknown(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("Service was not ready: {0}", e.into()),
                                )
                            }),
                        )
                    })?;
                let codec = tonic_prost::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static(
                    "/plugin.InfoProvider/GetPluginInfo",
                );
                let mut req = request.into_request();
                req.extensions_mut()
                    .insert(GrpcMethod::new("plugin.InfoProvider", "GetPluginInfo"));
                self.inner.unary(req, path, codec).await
            }
        }
    }
    /// Generated server implementations.
    pub mod info_provider_server {
        #![allow(
            unused_variables,
            dead_code,
            missing_docs,
            clippy::wildcard_imports,
            clippy::let_unit_value,
        )]
        use tonic::codegen::*;
        /// Generated trait containing gRPC methods that should be implemented for use with InfoProviderServer.
        pub trait InfoProvider: std::marker::Send + std::marker::Sync + 'static {
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn get_plugin_info<'life0, 'async_trait>(
                &'life0 self,
                request: tonic::Request<()>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = std::result::Result<
                            tonic::Response<super::PluginInfo>,
                            tonic::Status,
                        >,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
        }
        pub struct InfoProviderServer<T> {
            inner: Arc<T>,
            accept_compression_encodings: EnabledCompressionEncodings,
            send_compression_encodings: EnabledCompressionEncodings,
            max_decoding_message_size: Option<usize>,
            max_encoding_message_size: Option<usize>,
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug> ::core::fmt::Debug for InfoProviderServer<T> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "InfoProviderServer",
                    "inner",
                    &self.inner,
                    "accept_compression_encodings",
                    &self.accept_compression_encodings,
                    "send_compression_encodings",
                    &self.send_compression_encodings,
                    "max_decoding_message_size",
                    &self.max_decoding_message_size,
                    "max_encoding_message_size",
                    &&self.max_encoding_message_size,
                )
            }
        }
        impl<T> InfoProviderServer<T> {
            pub fn new(inner: T) -> Self {
                Self::from_arc(Arc::new(inner))
            }
            pub fn from_arc(inner: Arc<T>) -> Self {
                Self {
                    inner,
                    accept_compression_encodings: Default::default(),
                    send_compression_encodings: Default::default(),
                    max_decoding_message_size: None,
                    max_encoding_message_size: None,
                }
            }
            pub fn with_interceptor<F>(
                inner: T,
                interceptor: F,
            ) -> InterceptedService<Self, F>
            where
                F: tonic::service::Interceptor,
            {
                InterceptedService::new(Self::new(inner), interceptor)
            }
            /// Enable decompressing requests with the given encoding.
            #[must_use]
            pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.accept_compression_encodings.enable(encoding);
                self
            }
            /// Compress responses with the given encoding, if the client supports it.
            #[must_use]
            pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.send_compression_encodings.enable(encoding);
                self
            }
            /// Limits the maximum size of a decoded message.
            ///
            /// Default: `4MB`
            #[must_use]
            pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
                self.max_decoding_message_size = Some(limit);
                self
            }
            /// Limits the maximum size of an encoded message.
            ///
            /// Default: `usize::MAX`
            #[must_use]
            pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
                self.max_encoding_message_size = Some(limit);
                self
            }
        }
        impl<T, B> tonic::codegen::Service<http::Request<B>> for InfoProviderServer<T>
        where
            T: InfoProvider,
            B: Body + std::marker::Send + 'static,
            B::Error: Into<StdError> + std::marker::Send + 'static,
        {
            type Response = http::Response<tonic::body::Body>;
            type Error = std::convert::Infallible;
            type Future = BoxFuture<Self::Response, Self::Error>;
            fn poll_ready(
                &mut self,
                _cx: &mut Context<'_>,
            ) -> Poll<std::result::Result<(), Self::Error>> {
                Poll::Ready(Ok(()))
            }
            fn call(&mut self, req: http::Request<B>) -> Self::Future {
                match req.uri().path() {
                    "/plugin.InfoProvider/GetPluginInfo" => {
                        #[allow(non_camel_case_types)]
                        struct GetPluginInfoSvc<T: InfoProvider>(pub Arc<T>);
                        impl<T: InfoProvider> tonic::server::UnaryService<()>
                        for GetPluginInfoSvc<T> {
                            type Response = super::PluginInfo;
                            type Future = BoxFuture<
                                tonic::Response<Self::Response>,
                                tonic::Status,
                            >;
                            fn call(
                                &mut self,
                                request: tonic::Request<()>,
                            ) -> Self::Future {
                                let inner = Arc::clone(&self.0);
                                let fut = async move {
                                    <T as InfoProvider>::get_plugin_info(&inner, request).await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self
                            .accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let max_decoding_message_size = self.max_decoding_message_size;
                        let max_encoding_message_size = self.max_encoding_message_size;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let method = GetPluginInfoSvc(inner);
                            let codec = tonic_prost::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                )
                                .apply_max_message_size_config(
                                    max_decoding_message_size,
                                    max_encoding_message_size,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    _ => {
                        Box::pin(async move {
                            let mut response = http::Response::new(
                                tonic::body::Body::default(),
                            );
                            let headers = response.headers_mut();
                            headers
                                .insert(
                                    tonic::Status::GRPC_STATUS,
                                    (tonic::Code::Unimplemented as i32).into(),
                                );
                            headers
                                .insert(
                                    http::header::CONTENT_TYPE,
                                    tonic::metadata::GRPC_CONTENT_TYPE,
                                );
                            Ok(response)
                        })
                    }
                }
            }
        }
        impl<T> Clone for InfoProviderServer<T> {
            fn clone(&self) -> Self {
                let inner = self.inner.clone();
                Self {
                    inner,
                    accept_compression_encodings: self.accept_compression_encodings,
                    send_compression_encodings: self.send_compression_encodings,
                    max_decoding_message_size: self.max_decoding_message_size,
                    max_encoding_message_size: self.max_encoding_message_size,
                }
            }
        }
        /// Generated gRPC service name
        pub const SERVICE_NAME: &str = "plugin.InfoProvider";
        impl<T> tonic::server::NamedService for InfoProviderServer<T> {
            const NAME: &'static str = SERVICE_NAME;
        }
    }
    /// Generated client implementations.
    pub mod search_service_client {
        #![allow(
            unused_variables,
            dead_code,
            missing_docs,
            clippy::wildcard_imports,
            clippy::let_unit_value,
        )]
        use tonic::codegen::*;
        use tonic::codegen::http::Uri;
        pub struct SearchServiceClient<T> {
            inner: tonic::client::Grpc<T>,
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug> ::core::fmt::Debug for SearchServiceClient<T> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "SearchServiceClient",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl<T: ::core::clone::Clone> ::core::clone::Clone for SearchServiceClient<T> {
            #[inline]
            fn clone(&self) -> SearchServiceClient<T> {
                SearchServiceClient {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        impl SearchServiceClient<tonic::transport::Channel> {
            /// Attempt to create a new client by connecting to a given endpoint.
            pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
            where
                D: TryInto<tonic::transport::Endpoint>,
                D::Error: Into<StdError>,
            {
                let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
                Ok(Self::new(conn))
            }
        }
        impl<T> SearchServiceClient<T>
        where
            T: tonic::client::GrpcService<tonic::body::Body>,
            T::Error: Into<StdError>,
            T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
            <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
        {
            pub fn new(inner: T) -> Self {
                let inner = tonic::client::Grpc::new(inner);
                Self { inner }
            }
            pub fn with_origin(inner: T, origin: Uri) -> Self {
                let inner = tonic::client::Grpc::with_origin(inner, origin);
                Self { inner }
            }
            pub fn with_interceptor<F>(
                inner: T,
                interceptor: F,
            ) -> SearchServiceClient<InterceptedService<T, F>>
            where
                F: tonic::service::Interceptor,
                T::ResponseBody: Default,
                T: tonic::codegen::Service<
                    http::Request<tonic::body::Body>,
                    Response = http::Response<
                        <T as tonic::client::GrpcService<
                            tonic::body::Body,
                        >>::ResponseBody,
                    >,
                >,
                <T as tonic::codegen::Service<
                    http::Request<tonic::body::Body>,
                >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
            {
                SearchServiceClient::new(InterceptedService::new(inner, interceptor))
            }
            /// Compress requests with the given encoding.
            ///
            /// This requires the server to support it otherwise it might respond with an
            /// error.
            #[must_use]
            pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.inner = self.inner.send_compressed(encoding);
                self
            }
            /// Enable decompressing responses.
            #[must_use]
            pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.inner = self.inner.accept_compressed(encoding);
                self
            }
            /// Limits the maximum size of a decoded message.
            ///
            /// Default: `4MB`
            #[must_use]
            pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
                self.inner = self.inner.max_decoding_message_size(limit);
                self
            }
            /// Limits the maximum size of an encoded message.
            ///
            /// Default: `usize::MAX`
            #[must_use]
            pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
                self.inner = self.inner.max_encoding_message_size(limit);
                self
            }
            pub async fn search(
                &mut self,
                request: impl tonic::IntoRequest<super::SearchRequest>,
            ) -> std::result::Result<
                tonic::Response<super::SearchResponses>,
                tonic::Status,
            > {
                self.inner
                    .ready()
                    .await
                    .map_err(|e| {
                        tonic::Status::unknown(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("Service was not ready: {0}", e.into()),
                                )
                            }),
                        )
                    })?;
                let codec = tonic_prost::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static(
                    "/plugin.SearchService/Search",
                );
                let mut req = request.into_request();
                req.extensions_mut()
                    .insert(GrpcMethod::new("plugin.SearchService", "Search"));
                self.inner.unary(req, path, codec).await
            }
        }
    }
    /// Generated server implementations.
    pub mod search_service_server {
        #![allow(
            unused_variables,
            dead_code,
            missing_docs,
            clippy::wildcard_imports,
            clippy::let_unit_value,
        )]
        use tonic::codegen::*;
        /// Generated trait containing gRPC methods that should be implemented for use with SearchServiceServer.
        pub trait SearchService: std::marker::Send + std::marker::Sync + 'static {
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn search<'life0, 'async_trait>(
                &'life0 self,
                request: tonic::Request<super::SearchRequest>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = std::result::Result<
                            tonic::Response<super::SearchResponses>,
                            tonic::Status,
                        >,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
        }
        pub struct SearchServiceServer<T> {
            inner: Arc<T>,
            accept_compression_encodings: EnabledCompressionEncodings,
            send_compression_encodings: EnabledCompressionEncodings,
            max_decoding_message_size: Option<usize>,
            max_encoding_message_size: Option<usize>,
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug> ::core::fmt::Debug for SearchServiceServer<T> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "SearchServiceServer",
                    "inner",
                    &self.inner,
                    "accept_compression_encodings",
                    &self.accept_compression_encodings,
                    "send_compression_encodings",
                    &self.send_compression_encodings,
                    "max_decoding_message_size",
                    &self.max_decoding_message_size,
                    "max_encoding_message_size",
                    &&self.max_encoding_message_size,
                )
            }
        }
        impl<T> SearchServiceServer<T> {
            pub fn new(inner: T) -> Self {
                Self::from_arc(Arc::new(inner))
            }
            pub fn from_arc(inner: Arc<T>) -> Self {
                Self {
                    inner,
                    accept_compression_encodings: Default::default(),
                    send_compression_encodings: Default::default(),
                    max_decoding_message_size: None,
                    max_encoding_message_size: None,
                }
            }
            pub fn with_interceptor<F>(
                inner: T,
                interceptor: F,
            ) -> InterceptedService<Self, F>
            where
                F: tonic::service::Interceptor,
            {
                InterceptedService::new(Self::new(inner), interceptor)
            }
            /// Enable decompressing requests with the given encoding.
            #[must_use]
            pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.accept_compression_encodings.enable(encoding);
                self
            }
            /// Compress responses with the given encoding, if the client supports it.
            #[must_use]
            pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.send_compression_encodings.enable(encoding);
                self
            }
            /// Limits the maximum size of a decoded message.
            ///
            /// Default: `4MB`
            #[must_use]
            pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
                self.max_decoding_message_size = Some(limit);
                self
            }
            /// Limits the maximum size of an encoded message.
            ///
            /// Default: `usize::MAX`
            #[must_use]
            pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
                self.max_encoding_message_size = Some(limit);
                self
            }
        }
        impl<T, B> tonic::codegen::Service<http::Request<B>> for SearchServiceServer<T>
        where
            T: SearchService,
            B: Body + std::marker::Send + 'static,
            B::Error: Into<StdError> + std::marker::Send + 'static,
        {
            type Response = http::Response<tonic::body::Body>;
            type Error = std::convert::Infallible;
            type Future = BoxFuture<Self::Response, Self::Error>;
            fn poll_ready(
                &mut self,
                _cx: &mut Context<'_>,
            ) -> Poll<std::result::Result<(), Self::Error>> {
                Poll::Ready(Ok(()))
            }
            fn call(&mut self, req: http::Request<B>) -> Self::Future {
                match req.uri().path() {
                    "/plugin.SearchService/Search" => {
                        #[allow(non_camel_case_types)]
                        struct SearchSvc<T: SearchService>(pub Arc<T>);
                        impl<
                            T: SearchService,
                        > tonic::server::UnaryService<super::SearchRequest>
                        for SearchSvc<T> {
                            type Response = super::SearchResponses;
                            type Future = BoxFuture<
                                tonic::Response<Self::Response>,
                                tonic::Status,
                            >;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::SearchRequest>,
                            ) -> Self::Future {
                                let inner = Arc::clone(&self.0);
                                let fut = async move {
                                    <T as SearchService>::search(&inner, request).await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self
                            .accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let max_decoding_message_size = self.max_decoding_message_size;
                        let max_encoding_message_size = self.max_encoding_message_size;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let method = SearchSvc(inner);
                            let codec = tonic_prost::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                )
                                .apply_max_message_size_config(
                                    max_decoding_message_size,
                                    max_encoding_message_size,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    _ => {
                        Box::pin(async move {
                            let mut response = http::Response::new(
                                tonic::body::Body::default(),
                            );
                            let headers = response.headers_mut();
                            headers
                                .insert(
                                    tonic::Status::GRPC_STATUS,
                                    (tonic::Code::Unimplemented as i32).into(),
                                );
                            headers
                                .insert(
                                    http::header::CONTENT_TYPE,
                                    tonic::metadata::GRPC_CONTENT_TYPE,
                                );
                            Ok(response)
                        })
                    }
                }
            }
        }
        impl<T> Clone for SearchServiceServer<T> {
            fn clone(&self) -> Self {
                let inner = self.inner.clone();
                Self {
                    inner,
                    accept_compression_encodings: self.accept_compression_encodings,
                    send_compression_encodings: self.send_compression_encodings,
                    max_decoding_message_size: self.max_decoding_message_size,
                    max_encoding_message_size: self.max_encoding_message_size,
                }
            }
        }
        /// Generated gRPC service name
        pub const SERVICE_NAME: &str = "plugin.SearchService";
        impl<T> tonic::server::NamedService for SearchServiceServer<T> {
            const NAME: &'static str = SERVICE_NAME;
        }
    }
    /// Generated client implementations.
    pub mod source_provider_service_client {
        #![allow(
            unused_variables,
            dead_code,
            missing_docs,
            clippy::wildcard_imports,
            clippy::let_unit_value,
        )]
        use tonic::codegen::*;
        use tonic::codegen::http::Uri;
        pub struct SourceProviderServiceClient<T> {
            inner: tonic::client::Grpc<T>,
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug> ::core::fmt::Debug
        for SourceProviderServiceClient<T> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "SourceProviderServiceClient",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl<T: ::core::clone::Clone> ::core::clone::Clone
        for SourceProviderServiceClient<T> {
            #[inline]
            fn clone(&self) -> SourceProviderServiceClient<T> {
                SourceProviderServiceClient {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        impl SourceProviderServiceClient<tonic::transport::Channel> {
            /// Attempt to create a new client by connecting to a given endpoint.
            pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
            where
                D: TryInto<tonic::transport::Endpoint>,
                D::Error: Into<StdError>,
            {
                let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
                Ok(Self::new(conn))
            }
        }
        impl<T> SourceProviderServiceClient<T>
        where
            T: tonic::client::GrpcService<tonic::body::Body>,
            T::Error: Into<StdError>,
            T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
            <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
        {
            pub fn new(inner: T) -> Self {
                let inner = tonic::client::Grpc::new(inner);
                Self { inner }
            }
            pub fn with_origin(inner: T, origin: Uri) -> Self {
                let inner = tonic::client::Grpc::with_origin(inner, origin);
                Self { inner }
            }
            pub fn with_interceptor<F>(
                inner: T,
                interceptor: F,
            ) -> SourceProviderServiceClient<InterceptedService<T, F>>
            where
                F: tonic::service::Interceptor,
                T::ResponseBody: Default,
                T: tonic::codegen::Service<
                    http::Request<tonic::body::Body>,
                    Response = http::Response<
                        <T as tonic::client::GrpcService<
                            tonic::body::Body,
                        >>::ResponseBody,
                    >,
                >,
                <T as tonic::codegen::Service<
                    http::Request<tonic::body::Body>,
                >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
            {
                SourceProviderServiceClient::new(
                    InterceptedService::new(inner, interceptor),
                )
            }
            /// Compress requests with the given encoding.
            ///
            /// This requires the server to support it otherwise it might respond with an
            /// error.
            #[must_use]
            pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.inner = self.inner.send_compressed(encoding);
                self
            }
            /// Enable decompressing responses.
            #[must_use]
            pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.inner = self.inner.accept_compressed(encoding);
                self
            }
            /// Limits the maximum size of a decoded message.
            ///
            /// Default: `4MB`
            #[must_use]
            pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
                self.inner = self.inner.max_decoding_message_size(limit);
                self
            }
            /// Limits the maximum size of an encoded message.
            ///
            /// Default: `usize::MAX`
            #[must_use]
            pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
                self.inner = self.inner.max_encoding_message_size(limit);
                self
            }
            pub async fn get_sources(
                &mut self,
                request: impl tonic::IntoRequest<super::SourceRequest>,
            ) -> std::result::Result<
                tonic::Response<super::SourceResponse>,
                tonic::Status,
            > {
                self.inner
                    .ready()
                    .await
                    .map_err(|e| {
                        tonic::Status::unknown(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("Service was not ready: {0}", e.into()),
                                )
                            }),
                        )
                    })?;
                let codec = tonic_prost::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static(
                    "/plugin.SourceProviderService/GetSources",
                );
                let mut req = request.into_request();
                req.extensions_mut()
                    .insert(
                        GrpcMethod::new("plugin.SourceProviderService", "GetSources"),
                    );
                self.inner.unary(req, path, codec).await
            }
        }
    }
    /// Generated server implementations.
    pub mod source_provider_service_server {
        #![allow(
            unused_variables,
            dead_code,
            missing_docs,
            clippy::wildcard_imports,
            clippy::let_unit_value,
        )]
        use tonic::codegen::*;
        /// Generated trait containing gRPC methods that should be implemented for use with SourceProviderServiceServer.
        pub trait SourceProviderService: std::marker::Send + std::marker::Sync + 'static {
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn get_sources<'life0, 'async_trait>(
                &'life0 self,
                request: tonic::Request<super::SourceRequest>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = std::result::Result<
                            tonic::Response<super::SourceResponse>,
                            tonic::Status,
                        >,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
        }
        pub struct SourceProviderServiceServer<T> {
            inner: Arc<T>,
            accept_compression_encodings: EnabledCompressionEncodings,
            send_compression_encodings: EnabledCompressionEncodings,
            max_decoding_message_size: Option<usize>,
            max_encoding_message_size: Option<usize>,
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug> ::core::fmt::Debug
        for SourceProviderServiceServer<T> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "SourceProviderServiceServer",
                    "inner",
                    &self.inner,
                    "accept_compression_encodings",
                    &self.accept_compression_encodings,
                    "send_compression_encodings",
                    &self.send_compression_encodings,
                    "max_decoding_message_size",
                    &self.max_decoding_message_size,
                    "max_encoding_message_size",
                    &&self.max_encoding_message_size,
                )
            }
        }
        impl<T> SourceProviderServiceServer<T> {
            pub fn new(inner: T) -> Self {
                Self::from_arc(Arc::new(inner))
            }
            pub fn from_arc(inner: Arc<T>) -> Self {
                Self {
                    inner,
                    accept_compression_encodings: Default::default(),
                    send_compression_encodings: Default::default(),
                    max_decoding_message_size: None,
                    max_encoding_message_size: None,
                }
            }
            pub fn with_interceptor<F>(
                inner: T,
                interceptor: F,
            ) -> InterceptedService<Self, F>
            where
                F: tonic::service::Interceptor,
            {
                InterceptedService::new(Self::new(inner), interceptor)
            }
            /// Enable decompressing requests with the given encoding.
            #[must_use]
            pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.accept_compression_encodings.enable(encoding);
                self
            }
            /// Compress responses with the given encoding, if the client supports it.
            #[must_use]
            pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.send_compression_encodings.enable(encoding);
                self
            }
            /// Limits the maximum size of a decoded message.
            ///
            /// Default: `4MB`
            #[must_use]
            pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
                self.max_decoding_message_size = Some(limit);
                self
            }
            /// Limits the maximum size of an encoded message.
            ///
            /// Default: `usize::MAX`
            #[must_use]
            pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
                self.max_encoding_message_size = Some(limit);
                self
            }
        }
        impl<T, B> tonic::codegen::Service<http::Request<B>>
        for SourceProviderServiceServer<T>
        where
            T: SourceProviderService,
            B: Body + std::marker::Send + 'static,
            B::Error: Into<StdError> + std::marker::Send + 'static,
        {
            type Response = http::Response<tonic::body::Body>;
            type Error = std::convert::Infallible;
            type Future = BoxFuture<Self::Response, Self::Error>;
            fn poll_ready(
                &mut self,
                _cx: &mut Context<'_>,
            ) -> Poll<std::result::Result<(), Self::Error>> {
                Poll::Ready(Ok(()))
            }
            fn call(&mut self, req: http::Request<B>) -> Self::Future {
                match req.uri().path() {
                    "/plugin.SourceProviderService/GetSources" => {
                        #[allow(non_camel_case_types)]
                        struct GetSourcesSvc<T: SourceProviderService>(pub Arc<T>);
                        impl<
                            T: SourceProviderService,
                        > tonic::server::UnaryService<super::SourceRequest>
                        for GetSourcesSvc<T> {
                            type Response = super::SourceResponse;
                            type Future = BoxFuture<
                                tonic::Response<Self::Response>,
                                tonic::Status,
                            >;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::SourceRequest>,
                            ) -> Self::Future {
                                let inner = Arc::clone(&self.0);
                                let fut = async move {
                                    <T as SourceProviderService>::get_sources(&inner, request)
                                        .await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self
                            .accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let max_decoding_message_size = self.max_decoding_message_size;
                        let max_encoding_message_size = self.max_encoding_message_size;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let method = GetSourcesSvc(inner);
                            let codec = tonic_prost::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                )
                                .apply_max_message_size_config(
                                    max_decoding_message_size,
                                    max_encoding_message_size,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    _ => {
                        Box::pin(async move {
                            let mut response = http::Response::new(
                                tonic::body::Body::default(),
                            );
                            let headers = response.headers_mut();
                            headers
                                .insert(
                                    tonic::Status::GRPC_STATUS,
                                    (tonic::Code::Unimplemented as i32).into(),
                                );
                            headers
                                .insert(
                                    http::header::CONTENT_TYPE,
                                    tonic::metadata::GRPC_CONTENT_TYPE,
                                );
                            Ok(response)
                        })
                    }
                }
            }
        }
        impl<T> Clone for SourceProviderServiceServer<T> {
            fn clone(&self) -> Self {
                let inner = self.inner.clone();
                Self {
                    inner,
                    accept_compression_encodings: self.accept_compression_encodings,
                    send_compression_encodings: self.send_compression_encodings,
                    max_decoding_message_size: self.max_decoding_message_size,
                    max_encoding_message_size: self.max_encoding_message_size,
                }
            }
        }
        /// Generated gRPC service name
        pub const SERVICE_NAME: &str = "plugin.SourceProviderService";
        impl<T> tonic::server::NamedService for SourceProviderServiceServer<T> {
            const NAME: &'static str = SERVICE_NAME;
        }
    }
    /// Generated client implementations.
    pub mod link_resolver_service_client {
        #![allow(
            unused_variables,
            dead_code,
            missing_docs,
            clippy::wildcard_imports,
            clippy::let_unit_value,
        )]
        use tonic::codegen::*;
        use tonic::codegen::http::Uri;
        pub struct LinkResolverServiceClient<T> {
            inner: tonic::client::Grpc<T>,
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug> ::core::fmt::Debug for LinkResolverServiceClient<T> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "LinkResolverServiceClient",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl<T: ::core::clone::Clone> ::core::clone::Clone
        for LinkResolverServiceClient<T> {
            #[inline]
            fn clone(&self) -> LinkResolverServiceClient<T> {
                LinkResolverServiceClient {
                    inner: ::core::clone::Clone::clone(&self.inner),
                }
            }
        }
        impl LinkResolverServiceClient<tonic::transport::Channel> {
            /// Attempt to create a new client by connecting to a given endpoint.
            pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
            where
                D: TryInto<tonic::transport::Endpoint>,
                D::Error: Into<StdError>,
            {
                let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
                Ok(Self::new(conn))
            }
        }
        impl<T> LinkResolverServiceClient<T>
        where
            T: tonic::client::GrpcService<tonic::body::Body>,
            T::Error: Into<StdError>,
            T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
            <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
        {
            pub fn new(inner: T) -> Self {
                let inner = tonic::client::Grpc::new(inner);
                Self { inner }
            }
            pub fn with_origin(inner: T, origin: Uri) -> Self {
                let inner = tonic::client::Grpc::with_origin(inner, origin);
                Self { inner }
            }
            pub fn with_interceptor<F>(
                inner: T,
                interceptor: F,
            ) -> LinkResolverServiceClient<InterceptedService<T, F>>
            where
                F: tonic::service::Interceptor,
                T::ResponseBody: Default,
                T: tonic::codegen::Service<
                    http::Request<tonic::body::Body>,
                    Response = http::Response<
                        <T as tonic::client::GrpcService<
                            tonic::body::Body,
                        >>::ResponseBody,
                    >,
                >,
                <T as tonic::codegen::Service<
                    http::Request<tonic::body::Body>,
                >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
            {
                LinkResolverServiceClient::new(
                    InterceptedService::new(inner, interceptor),
                )
            }
            /// Compress requests with the given encoding.
            ///
            /// This requires the server to support it otherwise it might respond with an
            /// error.
            #[must_use]
            pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.inner = self.inner.send_compressed(encoding);
                self
            }
            /// Enable decompressing responses.
            #[must_use]
            pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.inner = self.inner.accept_compressed(encoding);
                self
            }
            /// Limits the maximum size of a decoded message.
            ///
            /// Default: `4MB`
            #[must_use]
            pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
                self.inner = self.inner.max_decoding_message_size(limit);
                self
            }
            /// Limits the maximum size of an encoded message.
            ///
            /// Default: `usize::MAX`
            #[must_use]
            pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
                self.inner = self.inner.max_encoding_message_size(limit);
                self
            }
            pub async fn resolve(
                &mut self,
                request: impl tonic::IntoRequest<super::ResolveRequest>,
            ) -> std::result::Result<
                tonic::Response<super::ResolveResponse>,
                tonic::Status,
            > {
                self.inner
                    .ready()
                    .await
                    .map_err(|e| {
                        tonic::Status::unknown(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("Service was not ready: {0}", e.into()),
                                )
                            }),
                        )
                    })?;
                let codec = tonic_prost::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static(
                    "/plugin.LinkResolverService/Resolve",
                );
                let mut req = request.into_request();
                req.extensions_mut()
                    .insert(GrpcMethod::new("plugin.LinkResolverService", "Resolve"));
                self.inner.unary(req, path, codec).await
            }
        }
    }
    /// Generated server implementations.
    pub mod link_resolver_service_server {
        #![allow(
            unused_variables,
            dead_code,
            missing_docs,
            clippy::wildcard_imports,
            clippy::let_unit_value,
        )]
        use tonic::codegen::*;
        /// Generated trait containing gRPC methods that should be implemented for use with LinkResolverServiceServer.
        pub trait LinkResolverService: std::marker::Send + std::marker::Sync + 'static {
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn resolve<'life0, 'async_trait>(
                &'life0 self,
                request: tonic::Request<super::ResolveRequest>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = std::result::Result<
                            tonic::Response<super::ResolveResponse>,
                            tonic::Status,
                        >,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait;
        }
        pub struct LinkResolverServiceServer<T> {
            inner: Arc<T>,
            accept_compression_encodings: EnabledCompressionEncodings,
            send_compression_encodings: EnabledCompressionEncodings,
            max_decoding_message_size: Option<usize>,
            max_encoding_message_size: Option<usize>,
        }
        #[automatically_derived]
        impl<T: ::core::fmt::Debug> ::core::fmt::Debug for LinkResolverServiceServer<T> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "LinkResolverServiceServer",
                    "inner",
                    &self.inner,
                    "accept_compression_encodings",
                    &self.accept_compression_encodings,
                    "send_compression_encodings",
                    &self.send_compression_encodings,
                    "max_decoding_message_size",
                    &self.max_decoding_message_size,
                    "max_encoding_message_size",
                    &&self.max_encoding_message_size,
                )
            }
        }
        impl<T> LinkResolverServiceServer<T> {
            pub fn new(inner: T) -> Self {
                Self::from_arc(Arc::new(inner))
            }
            pub fn from_arc(inner: Arc<T>) -> Self {
                Self {
                    inner,
                    accept_compression_encodings: Default::default(),
                    send_compression_encodings: Default::default(),
                    max_decoding_message_size: None,
                    max_encoding_message_size: None,
                }
            }
            pub fn with_interceptor<F>(
                inner: T,
                interceptor: F,
            ) -> InterceptedService<Self, F>
            where
                F: tonic::service::Interceptor,
            {
                InterceptedService::new(Self::new(inner), interceptor)
            }
            /// Enable decompressing requests with the given encoding.
            #[must_use]
            pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.accept_compression_encodings.enable(encoding);
                self
            }
            /// Compress responses with the given encoding, if the client supports it.
            #[must_use]
            pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
                self.send_compression_encodings.enable(encoding);
                self
            }
            /// Limits the maximum size of a decoded message.
            ///
            /// Default: `4MB`
            #[must_use]
            pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
                self.max_decoding_message_size = Some(limit);
                self
            }
            /// Limits the maximum size of an encoded message.
            ///
            /// Default: `usize::MAX`
            #[must_use]
            pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
                self.max_encoding_message_size = Some(limit);
                self
            }
        }
        impl<T, B> tonic::codegen::Service<http::Request<B>>
        for LinkResolverServiceServer<T>
        where
            T: LinkResolverService,
            B: Body + std::marker::Send + 'static,
            B::Error: Into<StdError> + std::marker::Send + 'static,
        {
            type Response = http::Response<tonic::body::Body>;
            type Error = std::convert::Infallible;
            type Future = BoxFuture<Self::Response, Self::Error>;
            fn poll_ready(
                &mut self,
                _cx: &mut Context<'_>,
            ) -> Poll<std::result::Result<(), Self::Error>> {
                Poll::Ready(Ok(()))
            }
            fn call(&mut self, req: http::Request<B>) -> Self::Future {
                match req.uri().path() {
                    "/plugin.LinkResolverService/Resolve" => {
                        #[allow(non_camel_case_types)]
                        struct ResolveSvc<T: LinkResolverService>(pub Arc<T>);
                        impl<
                            T: LinkResolverService,
                        > tonic::server::UnaryService<super::ResolveRequest>
                        for ResolveSvc<T> {
                            type Response = super::ResolveResponse;
                            type Future = BoxFuture<
                                tonic::Response<Self::Response>,
                                tonic::Status,
                            >;
                            fn call(
                                &mut self,
                                request: tonic::Request<super::ResolveRequest>,
                            ) -> Self::Future {
                                let inner = Arc::clone(&self.0);
                                let fut = async move {
                                    <T as LinkResolverService>::resolve(&inner, request).await
                                };
                                Box::pin(fut)
                            }
                        }
                        let accept_compression_encodings = self
                            .accept_compression_encodings;
                        let send_compression_encodings = self.send_compression_encodings;
                        let max_decoding_message_size = self.max_decoding_message_size;
                        let max_encoding_message_size = self.max_encoding_message_size;
                        let inner = self.inner.clone();
                        let fut = async move {
                            let method = ResolveSvc(inner);
                            let codec = tonic_prost::ProstCodec::default();
                            let mut grpc = tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                    accept_compression_encodings,
                                    send_compression_encodings,
                                )
                                .apply_max_message_size_config(
                                    max_decoding_message_size,
                                    max_encoding_message_size,
                                );
                            let res = grpc.unary(method, req).await;
                            Ok(res)
                        };
                        Box::pin(fut)
                    }
                    _ => {
                        Box::pin(async move {
                            let mut response = http::Response::new(
                                tonic::body::Body::default(),
                            );
                            let headers = response.headers_mut();
                            headers
                                .insert(
                                    tonic::Status::GRPC_STATUS,
                                    (tonic::Code::Unimplemented as i32).into(),
                                );
                            headers
                                .insert(
                                    http::header::CONTENT_TYPE,
                                    tonic::metadata::GRPC_CONTENT_TYPE,
                                );
                            Ok(response)
                        })
                    }
                }
            }
        }
        impl<T> Clone for LinkResolverServiceServer<T> {
            fn clone(&self) -> Self {
                let inner = self.inner.clone();
                Self {
                    inner,
                    accept_compression_encodings: self.accept_compression_encodings,
                    send_compression_encodings: self.send_compression_encodings,
                    max_decoding_message_size: self.max_decoding_message_size,
                    max_encoding_message_size: self.max_encoding_message_size,
                }
            }
        }
        /// Generated gRPC service name
        pub const SERVICE_NAME: &str = "plugin.LinkResolverService";
        impl<T> tonic::server::NamedService for LinkResolverServiceServer<T> {
            const NAME: &'static str = SERVICE_NAME;
        }
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = async {
        let greeter = Info::default();
        Ok(())
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
