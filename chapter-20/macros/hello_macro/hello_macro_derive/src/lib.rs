// Since the `hello_macro` crate and the `hello_macro_derive` crate are closely related,
// we'll create the `hello_macro_derive` crate under the `hello_macro` crate
// Note that if we change the trait defintion in `hello_macro`, we'll have to change the implementation of the procedural macro in `hello_macro_derive`
// The two crates will need to be published seperately, and programmers will need to add both as dependencies and bring them both into scope

// Note the 3 new crates:
// - `proc_macro` comes with Rust, and is the compiler's API that allows reading and manipulation of Rust code from our code
// - `syn` parses Rust code from a string into a data structure that we can perform operations on
// - `quote` turns `syn` data structures back into Rust code
use proc_macro::TokenStream;
use quote::quote;
use syn;

// The `hello_macro_derive` function will be called when a user specifies `#[derive(HelloMacro)]` on a type
// This is because of the `proc_macro_derive` annotation and specified the name `HelloMacro` (the trait name)
// This is the convention most procedural macros follow
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Note the split between hello_macro_derive() function (which parses the `TokenStream`)
    // and the impl_hello_macro() function (which transforms the syntax tree)
    // The code in the outer function will be the same for almost every procedural macro
    // The code in the inner function will be different depending on the procedural macro's purpose

    // Construct a representation of Rust code as a syntax tree that we can manupulate

    // The `hello_macro_derive` function first converts the `input` from a `TokenStream` to a data structure that we can interpet and perform operations on.
    // The `parse` function in `syn` takes a `TokenStream` and returns a `DeriveInput` struct respresenting the parsed Rust code
    // Note the usage of unwrap() here, it is necessary for the procedural macro to panic on errors because we must return `TokenStream` rather than `Result`,
    // in order to conform to the procedural macro API
    // In production code, use `panic!` or expect() to provide an error message
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // get an `Ident` struct instance with the name (identifier) of the annotated type using `ast.ident`
    let name = &ast.ident;
    // The `quote!` macro provides templating mechanics
    // We can enter `#name` and `quote!` will replace it with the value in the variable `name`
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // The `stringify!` macro takes a Rust expression and at compile time, turns the expression into a string literal
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
