#![recursion_limit = "256"]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_attribute]
pub fn fce(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fn_def = parse_macro_input!(item as ItemFn);
    let fn_name = fn_def.ident.to_string();
    let original_fn_body = fn_def.block.clone();
    let fn_body = parse_quote! {{
        use failure::ResultExt;
        let mut fn_closure = move || #original_fn_body;
        let ret:Result<_,Error> = fn_closure();
        let ok_val = ret.context(format!("call {}() err",#fn_name))?;//call the origin function and add function name into context;
        Ok(ok_val)
    }};
    fn_def.block = Box::new(fn_body);
    TokenStream::from(quote!(#fn_def))
}
