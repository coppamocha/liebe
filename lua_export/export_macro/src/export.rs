// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
use export_core::ExportedFn;
use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    ItemFn, LitStr, ReturnType, Token, parse::Parser, parse_macro_input, punctuated::Punctuated,
};

pub fn mlua_export(attr: TokenStream, item: TokenStream) -> TokenStream {
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

    let arg_types: Vec<_> = sig
        .inputs
        .iter()
        .filter_map(|arg| {
            match arg {
                syn::FnArg::Typed(pat_ty) => Some(&*pat_ty.ty),
                _ => None, // skip `self`
            }
        })
        .collect();

    let ret_type = match &sig.output {
        ReturnType::Type(_, ty) => quote! { #ty },
        ReturnType::Default => quote! { () },
    };

    let fn_type = quote! {
        fn(#(#arg_types),*) -> #ret_type
    };

    let type_id = quote! {
        ::std::any::TypeId::of::<#fn_type>()
    };

    let register = quote! {
        inventory::submit! {
            ExportedFn {
                module: #mod_name,
                name: #func_name,
                function: #name as *const (),
                type_id: #type_id,
            }
        }
    };
    let expanded = quote! {
        #vis #sig {
            #block
        }
    };

    TokenStream::from(expanded)
}
