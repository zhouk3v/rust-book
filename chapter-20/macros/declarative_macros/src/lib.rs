// "Macro" refers to a family of features in Rust:
// - Declarative macros with `macro_rules!`
// - Three kinds of procedural macros:
//     - Custom `#[derive]` macros that specify code added with the `derive` attribute used on structs and enums
//     - Attribute-like macros that define custom attributes usable on any items
//     - Function-like macros that look like function calls but operate on the tokens specified as their argument

//
// The Difference between Macros and Functions
//

// Macros are a way of writing code that writes other code (metaprogramming)
// all macros used before expand to produce more code than the code written manually

// Macros have some additional powers that functions don't
// A function signature must declare the number and type of parameters the functions has,
// Macros can take a variable number of parameters
// Macros are also expanded before the compiler interprets the meaning of the code,
// so a macro can, for example, implement a trait on a given type
// (functions can't, since it's called at runtime and a trait needs to be implemented at compile time)

// The downside of macros over functions is that macro definitions are more complex, since it's Rust code that writes Rust code
// Because of this, macro definitions are generally more difficult to read, understand and maintain than function definitions

// Another difference is between macros and functions is that you must define macros or bring them into scope before you call them,
// compared to functions that can be defined anywhere and called anywhere

//
// Declarative Macros with `macro_rules!` for General Metaprogramming
//

// Declarative macros are similar to a `match` expression
// Macros compare values to patterns that are associated with particular code, where the value is the literal Rust source code passed to the macro
// The patterns are compared with the structure of that source code, and the code associated with each pattern,
// when matched, replaces the code passed to the macro.
// Note that this all happens during compilation

// To define a macro, use the `macro_rules!` construct
// E.g. the `vec!` macro (simplified)

// This annotation indicates that this macro should be made available whenever the crate in which the macro is defined is brought into scope
#[macro_export]
// Start the macro definition with `macro_rules!` and then name the macro without the exclamation mark,
// followed by curly braces with the body of the macro definition
macro_rules! vec_def {
    // The macro structure is similar to a `match` expression (arms with patterns and the block of code associated with it)
    // If the pattern matches, the associated block of code will be emitted

    // Valid pattern syntax in macro definitions are matched against Rust code structure rather than values
    // Eg: declarative macros can match against expression (`expr`), types (`ty`) and entire items (`item`)

    // First, a set of parentheses encompass the whole pattern
    // The dollar sign `$` is used to decalre a variable in the macro system that will contain the Rust code matching the pattern
    // In the first macro variable, `$($x:expr )` a second set of parentheses captures values that match the pattern within the parentheses for use in the replacement code
    // Within the second set of parentheses, `$x:expr` matches any Rust expression and gives the expression the name `$x`

    // The comma after `$()` indicates that a literal comma seperator character could optionally appear after the code that matches the code in `$()`
    // The `*` specifies that the pattern matches zero or more of whater precedes the `*`.

    // When calling the macro with `vec![1,2,3];`, the `$x` pattern matches three times with the three expressions `1`, `2` and `3`
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            // `temp_vec.push()` within the `$()*` is generated for each part that matches `$()` in the pattern zero or more times
            // The `$x` is replaced with each expression matched
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };

    // When we call the macro with `vec![1,2,3]`, the code generated that replaces this macro call is
    /*
    {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
    }
     */
}
