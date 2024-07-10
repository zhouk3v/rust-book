Attribute-like macros allow you to create new attributes
In contrast to `derive` macros, which only work on structs and enums, attributes can be applied to other items like functions

Example: `route` attribute, which annotates functions when using a web application framework:

```
#[route(GET, "/")]
fn index() {
```

The `#[route]` attribute would be defined by the framework as a procedural macro. The signature of the macro definition function looks like this:

```
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

We have two parameters of type `TokenStream`

- The first is for the contents of the attribute (the `GET, "/"` part)
- The second is the body of the item the attribute is attached to (in this case, `fn index() {}`) and the rest of the function body
