//
// Commenting Contained Items
//

// The `//!` style of doc comment adds documents to the item that contains the comments rather than to the items following the comments
// (i.e. the root page with links to all functions/structs in the library)
// Typical convention is to use these doc comments inside the crate root file or module as a whole
// In other words, this documentation summarizes the point of the library

//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
//!

//
// Making useful documentation comments
//

// Rust has a comment type for documentation, that will generate HTML docs (documentation comment)
// The HTML will display info on how to use the crate rather than how the crate is implemented

// Documentation comments use three slashes, and support markdown notation for formatting

/// Adds one to the number given
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Use `cargo doc` to generate the HTML documentation, which will be put under target/doc
// You can add the --open option to view the result in a web browser

//
// Commonly Used Sections
//

// Panics - The scenarios which the function being documented could panic

// Errors - If the function returns a Result,
// describing the kinds of errors that might occur and what conditions might cause those errors to be returned can be
// can be helpful to callers so they can handle it in their own appropiate way

// Safety - If the function is unsafe to call, there should be a section explaining why the function is unsafe
// and covering the invariants that the function expects callers to uphold

//
// Documentation Comments as Tests
//

// Running `cargo test`` will run code examples in the documentations as tests.
// This is helpful when updating examples in response to code changes
