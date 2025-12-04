use crate::handler::{Invocation, args::HandlerType};
use quote::quote;

pub fn generate_info(inv: &Invocation) -> proc_macro2::TokenStream {
    let name = &inv.function.sig.ident;
    match inv.args.handler_type {
        HandlerType::Search => search_callback(inv, name),
        HandlerType::SourceProvider => source_provider_callback(inv, name),
        HandlerType::LinkResolver => link_resolver_callback(inv, name),
    }
}

pub fn search_callback(inv: &Invocation, name: &proc_macro2::Ident) -> proc_macro2::TokenStream {
    quote! {
        ::plugin_sdk::handler::HandlerInfo::Search(::plugin_sdk::handler::SearchHandler {
            callback: |ctx, request| Box::pin(async move { #name(ctx, request).await }),
        })
    }
}

pub fn source_provider_callback(
    inv: &Invocation,
    name: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    quote! {
        ::plugin_sdk::handler::HandlerInfo::SourceProvider(::plugin_sdk::handler::SourceProviderHandler {
            callback: |ctx, request| Box::pin(async move { #name(ctx, request).await }),
        })
    }
}

pub fn link_resolver_callback(
    inv: &Invocation,
    name: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    quote! {
        ::plugin_sdk::handler::oHandlerInfo::LinkResolver(::plugin_sdk::handler::LinkResolverHandler {
            callback: |ctx, request| Box::pin(async move { #name(ctx, request).await }),
        })
    }
}
