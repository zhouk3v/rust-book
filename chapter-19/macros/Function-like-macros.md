Function-like macros define macros that look like function calls.
Similarly to `macro_rules!` macros, they are more flexible than functions: They can take an unknown number of arguments

Function-like macros take a `TokenStream` parameter and their definition manipulates that `TokenStream` using Rust code

Example: an `sql!` macro:

```
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

This macro would parse the SQL statement inside it and check the syntax

The `sql!` macro would be defined like this:

```
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

The definition is similar to the custom derive macro's signature, we receive the tokens that are inside the parentheses and return the code we wanted to generate
