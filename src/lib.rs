#![recursion_limit = "256"]

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse::*, *};

#[proc_macro_attribute]
pub fn fce(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fn_def = parse_macro_input!(item as ItemFn);
    let fn_name_str = fn_def.ident.to_string();

    let mut wrapped_fn_def = fn_def.clone();
    let wraped_fn_name_str = format!("{}_hide", fn_name_str);
    let wraped_fn_name = Ident::new(&wraped_fn_name_str, Span::call_site());
    wrapped_fn_def.ident = wraped_fn_name.clone(); //decorate function

    let fn_body = parse_quote! {{
        use failure::ResultExt;
        #wrapped_fn_def;
        let res = #wraped_fn_name().context(format!("call {}() err",#fn_name_str))?;//call the origin function and add function name into context;
        Ok(res)
    }};

    fn_def.block = Box::new(fn_body);
    TokenStream::from(quote!(#fn_def))
}
