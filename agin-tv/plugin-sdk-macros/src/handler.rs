use proc_macro::TokenStream;
use syn::spanned::Spanned;

use crate::handler::args::HandlerArgs;

pub mod args;

pub fn handler(
    args: HandlerArgs,
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
    if args.providers.is_empty() {
        let err_msg = "you must specify at least one provider";
        return Err(syn::Error::new(proc_macro2::Span::call_site(), err_msg).into());
    }

    todo!()
}

pub struct Invocation {
    function: syn::ItemFn,
    args: HandlerArgs,
}

fn generate_command(mut inv: Invocation) -> Result<proc_macro2::TokenStream, darling::Error> {
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

    let function_name = inv
        .function
        .sig
        .ident
        .to_string()
        .trim_start_matches("r#")
        .to_string();

    let function_ident =
        std::mem::replace(&mut inv.function.sig.ident, syn::parse_quote! { inner });
    let function_generics = &inv.function.sig.generics;
    let function_visibility = &inv.function.vis;
    let function = &inv.function;

    Ok(quote::quote! {
        #[allow(clippy::str_to_string)]
        #function_visibility fn #function_ident #function_generics() -> ::plugin_sdk::handler::HandlerMetadata<
            <#ctx_type_with_static as plugin_sdk::_GetGenerics>::T,
        > {
            #function

            ::plugin_sdk::handler::HandlerMetadata {
                providers: vec![],
            }
        }
    })
}
