# Rust Proc Macro Target Demo
This repo demonstrates that the Rust compiler does not take into account the target architecture when running proc macros.

in `/src` directory is a test with proc macro generating a `TEST_STRING` that gets printed, and prints out a message from the target architecture.. in the `proc-macro-core`, the test unit in there will give the correct output, but this test calls the function directly, and not the macro.

the implementation for the macro is just a few configs that SHOULD give a different result, with a different architecture, but I have had no luck with that.

