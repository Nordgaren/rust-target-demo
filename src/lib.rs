pub use proc_macro_derive::demo_proc_macro;


#[test]
fn import_test() {
    demo_proc_macro!();

    println!("{}", TEST_STRING);
}