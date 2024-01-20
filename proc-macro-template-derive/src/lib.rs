#![doc = include_str!("../README.md")]

use proc_macro_template_core::proc_macro_impl;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn proc_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    proc_macro_impl(args.into(), input.into()).into()
}