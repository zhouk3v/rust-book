//
// How to Write a Custom `derive` macro
//

// HelloMacro trait and its associated function
pub trait HelloMacro {
    // Note that we can't yet provide the hello_macro function with a default implementation that prints the name of the type that the trait is implemented on
    // Rust does not have reflection capabilities, so it can't look up the type's name at runtime
    fn hello_macro();
}
