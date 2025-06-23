use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn mlua_export(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse arguments like #[my_macro("a", "b")]
    let args = parse_macro_input!(attr as AttributeArgs);

    // Extract the two string literals
    let strings: Vec<String> = args
        .iter()
        .filter_map(|arg| {
            if let NestedMeta::Lit(Lit::Str(s)) = arg {
                Some(s.value())
            } else {
                None
            }
        })
        .collect();

    if strings.len() != 2 {
        return syn::Error::new_spanned(&args[0], "Expected exactly 2 string arguments")
            .to_compile_error()
            .into();
    }

    let arg1 = &strings[0];
    let arg2 = &strings[1];

    // Parse the function this macro is attached to
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let sig = &input_fn.sig;
    let block = &input_fn.block;
    let vis = &input_fn.vis;

    let output = quote! {
        #vis #sig {
            println!("Macro args: \"{}\", \"{}\"", #arg1, #arg2);
            #block
        }
    };

    output.into()
}
