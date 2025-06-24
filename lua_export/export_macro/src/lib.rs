// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
mod export;

#[proc_macro_attribute]
pub fn lua_export(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    export::lua_export(attr, item)
}
