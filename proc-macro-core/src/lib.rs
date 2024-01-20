#![doc = include_str!("../README.md")]

mod tests;

use proc_macro2::TokenStream;
use quote::quote;


pub fn proc_macro_impl(args: TokenStream) -> TokenStream {
    #[cfg(target_arch = "x86")]
    quote!{
        const TEST_STRING: &str = "Hello, fom x86!";
    }

    #[cfg(target_arch = "x86_64")]
    quote!{
        const TEST_STRING: &str = "Hello, fom x86_64!";
    }
}
