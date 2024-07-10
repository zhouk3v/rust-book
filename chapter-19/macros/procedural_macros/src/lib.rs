//
// Procedural Macros for Generating Code from Attributes
//

// Procedural macros act more like a function
// They accept some code as an input, operate on that code, and produce some code as an output
// The three kinds of procedural macros are custom derive, attribute-like and function-like, and all work in a similar fashion

// When creating procedural macros, the definitions must reside in their own crate with a special crate type.

use proc_macro;

// `some_attribute` is a placeholder for using a specific macro variety
#[some_attribute]
// The function that defines a procedural macro takes a `TokenStream` as an input and produces a `TokenStream` as an output
// `TokenStream` (from the `proc_macro` crate) represents a sequence of tokens
// The source code that the macro is operating on makes up the input `TokenStream`
// The code that the macro produces is the output `TokenStream`
// The function also has an attribute attached to it that specifies which kind of procedural macro we're creating
// We can have multiple kinds of procedural macros in the same crate
pub fn some_name(input: TokenStream) -> TokenStream {}
