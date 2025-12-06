use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;

use crate::handler::{
    args::{HandlerArgs, RawHandlerArgs},
    info::generate_info,
};

pub mod args;
pub mod info;

pub fn handler(
    args: RawHandlerArgs,
    mut function: syn::ItemFn,
) -> Result<TokenStream, darling::Error> {
    if function.sig.asyncness.is_none() {
        return Err(syn::Error::new(function.sig.span(), "handler function must be async").into());
    }

    if function.sig.output == syn::ReturnType::Default {
        return Err(syn::Error::new(
            function.sig.span(),
            "handler function must return Result<(), ...>",
        )
        .into());
    }

    // Verify that handler registers providers
    if args.supports.is_empty() {
        let err_msg = "you must specify at least one provider";
        return Err(syn::Error::new(proc_macro2::Span::call_site(), err_msg).into());
    }

    let handler_args = match HandlerArgs::try_from(args) {
        Ok(handler_args) => handler_args,
        Err(e) => {
            return Err(syn::Error::new(proc_macro2::Span::call_site(), e).into());
        }
    };

    let inv = Invocation {
        function,
        args: handler_args,
    };

    Ok(TokenStream::from(generate_handler(inv)?))
}

pub struct Invocation {
    function: syn::ItemFn,
    args: HandlerArgs,
}

// TODO: Better error handling
fn generate_handler(mut inv: Invocation) -> Result<proc_macro2::TokenStream, darling::Error> {
    let ctx_type = match inv.function.sig.inputs.first() {
        Some(syn::FnArg::Typed(syn::PatType { ty, .. })) => &**ty,
        _ => {
            return Err(
                syn::Error::new(inv.function.sig.span(), "expected a Context parameter").into(),
            );
        }
    };
    // Needed because we're not allowed to have lifetimes in the hacky use case below
    let ctx_type_with_static =
        syn::fold::fold_type(&mut crate::util::AllLifetimesToStatic, ctx_type.clone());

    let current_ident = inv.function.sig.ident.clone();
    let inner_ident = syn::Ident::new(&format!("{current_ident}_inner"), current_ident.span());

    let function_ident = std::mem::replace(&mut inv.function.sig.ident, inner_ident);
    let function_generics = &inv.function.sig.generics;
    let function_visibility = inv.function.vis.clone();
    inv.function.vis = syn::Visibility::Inherited;
    let function = &inv.function;

    let providers = inv.args.supports.iter().map(|lit| {
        quote! { #lit.to_string() }
    });
    let info = generate_info(&inv);

    Ok(quote! {
        #function
        #[allow(clippy::str_to_string)]
        #function_visibility fn #function_ident #function_generics() -> ::plugin_sdk::handler::HandlerMetadata<
            <#ctx_type_with_static as plugin_sdk::_GetGenerics>::T
        > {

            ::plugin_sdk::handler::HandlerMetadata {
                providers: vec![#(#providers),*],
                info: #info,
            }
        }
    })
}
