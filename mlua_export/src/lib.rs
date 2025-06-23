mod export;

use export::mlua_export;

#[proc_macro_attribute]
pub fn mlua_export(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    mlua_export(attr, item)
}
