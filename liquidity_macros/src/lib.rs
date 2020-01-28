extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{ItemFn, AttributeArgs, NestedMeta, Lit};
use quote::quote;

/// This is used internally only, so it's really really dumb and doesn't do error handling at all
#[proc_macro_attribute]
pub fn authorized(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as AttributeArgs);
    let input = syn::parse_macro_input!(input as ItemFn);

    let permission = if args.is_empty() { None } else {
        if let Some(NestedMeta::Lit(Lit::Str(literal))) = args.first() {
            Some(literal.value())
        } else { panic!("Argument must be a string literal") }
    };

    let other_attr = input.attrs;
    let visibility = input.vis;
    let signature = input.sig;
    let statements = input.block.stmts;

    let check = permission.map(|key| {
        quote! {
            liquidity::permissions::check(#key, context.user())?;
        }
    }).unwrap_or_else(|| {
        quote! {
            context.user().as_ref().ok_or(PermissionError::NotLoggedIn)?;
        }
    });

    let tokens = quote! {
        #(#other_attr)*
        #visibility #signature {
            #check
            #(#statements)*
        }
    };

    tokens.into()
}