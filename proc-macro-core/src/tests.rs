#![cfg(test)]

use crate::proc_macro_impl;
use quote::quote;

#[test]
fn core_test() {
    let after = proc_macro_impl(quote!());
    println!("{}", after);
    assert_ne!(
        after.to_string(),
        ""
    );
}
