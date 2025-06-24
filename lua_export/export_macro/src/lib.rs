// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
mod export;
use export::mlua_export as export_alias;

#[proc_macro_attribute]
pub fn mlua_export(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    export_alias(attr, item)
}
