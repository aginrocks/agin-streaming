use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemImpl, parse_macro_input};

#[derive(Debug, FromMeta)]
struct Args {}

#[proc_macro_attribute]
pub fn service(attr: TokenStream, item: TokenStream) -> TokenStream {
    // TODO: parse args

    let input_impl = parse_macro_input!(item as ItemImpl);
    let self_type = &input_impl.self_ty;

    let trait_path = match &input_impl.trait_ {
        Some((_, path, _)) => path, // this is the trait path
        None => {
            return syn::Error::new_spanned(
                &input_impl.self_ty,
                "This macro must be applied to `impl Something for Type` (trait impls only)",
            )
            .to_compile_error()
            .into();
        }
    };

    let trait_ident = &trait_path.segments.last().unwrap().ident;
    let server_ident = syn::Ident::new(&format!("{}Server", trait_ident), trait_ident.span());

    let mut server_path = trait_path.clone();
    server_path.segments.last_mut().unwrap().ident = server_ident;

    let metadata_body = quote! { todo!("Meta") };

    let agin_trait = quote! { crate::AginService };
    let service_ty = quote! { plugin_sdk::plugin::Agin };

    todo!()
}
