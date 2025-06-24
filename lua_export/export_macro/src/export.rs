// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{ItemFn, LitStr, Token, parse::Parser, parse_macro_input, punctuated::Punctuated};

pub fn lua_export(attr: TokenStream, item: TokenStream) -> TokenStream {
    let res = Punctuated::<LitStr, Token![,]>::parse_terminated.parse2(attr.into());
    let Ok(args) = res else {
        return res
            .err()
            .unwrap()
            .into_compile_error()
            .into_token_stream()
            .into();
    };

    if args.len() != 2 {
        return syn::Error::new_spanned(
            quote! { #[mlua_export(...)] },
            "Expected exactly two string arguments",
        )
        .to_compile_error()
        .into();
    }

    let mod_name = &args[0].value();
    let func_name = &args[1].value();

    let input_fn = parse_macro_input!(item as ItemFn);
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let name = &sig.ident;
    let block = &input_fn.block;

    let register = quote! {
        ::lua_export::inventory::submit! {
            ::lua_export::ExportedFn {
                module: #mod_name,
                name: #func_name,
                function: #name as *const (),
            }
        }
    };
    let expanded = quote! {
        #vis #sig {
            #block
        }
        #register
    };
    TokenStream::from(expanded)
}
