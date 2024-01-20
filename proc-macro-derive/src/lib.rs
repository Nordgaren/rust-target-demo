#![doc = include_str!("../README.md")]

use proc_macro_core::proc_macro_impl;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;

#[proc_macro_error]
#[proc_macro]
pub fn demo_proc_macro(args: TokenStream) -> TokenStream {
    proc_macro_impl(args.into()).into()

    // #[cfg(target_arch = "x86")]
    // let q =quote!{
    //     const TEST_STRING: &str = "Hello, fom x86_64!";
    // };
    //
    // #[cfg(target_arch = "x86_64")]
    // let q =quote!{
    //     const TEST_STRING: &str = "Hello, fom x86_64!";
    // };
    //
    // q.into()
}