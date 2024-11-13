extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn retry(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let mut arg_iter = args.into_iter();
    let retries = if let Some(lit) = arg_iter.next() {
        lit.to_string().parse::<u64>().unwrap()
    } else {
        3
    };

    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;
    let fn_sig = &input_fn.sig;
    let fn_vis = &input_fn.vis;

    let expanded = quote! {
        #fn_vis #fn_sig {
            let mut attempt = 0;
            loop {
                attempt += 1;
                match (|| #fn_body )() {
                    Ok(result) => return Ok(result),
                    Err(err) if attempt < #retries => {
                    },
                    Err(err) => {
                        panic!("Function {} failed after {} retries: {}", stringify!(#fn_name), #retries, err);
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn retry_optional(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let mut arg_iter = args.into_iter();
    let retries = if let Some(lit) = arg_iter.next() {
        lit.to_string().parse::<u64>().unwrap()
    } else {
        3
    };

    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;
    let fn_sig = &input_fn.sig;
    let fn_vis = &input_fn.vis;

    let expanded = quote! {
        #fn_vis #fn_sig {
            let mut attempt = 0;
            loop {
                attempt += 1;
                match (|| #fn_body )() {
                    Some(result) => return Some(result),
                    None if attempt < #retries => {
                    },
                    None => {
                        panic!("Function {} failed after {} retries: None", stringify!(#fn_name), #retries);
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}
