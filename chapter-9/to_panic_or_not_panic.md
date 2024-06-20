When code panics, there is no way to recover. You make the decision that a situation is unrecoverable on behalf on the caller.

With Result, you provide an oppertunity for the calling code to recover, where they can deal with the error or panic themselves. As a result, returning Result is a good default option

There are three main reasons to panic (mainly in the form of unwrap() and expect()):

- Example code, to indicate that the example can fail, and so it is on the user to do error handling
- Prototyping, to leave markers that error handling needs to be implemented later
- Tests, to indicate test failure

unwrap() and expect() can also be used if the logic guarantees that a Result will always be Ok, but the compiler can't see it. For example, calling parse() on a hard-coded string:

```
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

```

# Guidelines for error handling

Code should panic if it's in a bad state.

A bad state means some assumption or invairant is broken (e.g. invalid values are passed into the code) plus one or more of the following cases

- The bad state is something unexpected
- The code is at a point were it needs to rely on not being in a bad state, rather than checking at every step
- There is not a good way to encode this information in the types that are currently used

In cases where continuing with invalid values can be insecure or harmful, the best choice is to call panic and alert the person using your library that there is a bug in their code.

Calling panic is also appropiate if a library returns an invalid state which has no way of fixing

When a failure is expected, it is better to return a result (e.g. parser being given malformed data or a HTTP request htting a rate limit). In this case, returning a Result means that failure is an expected possiblity which the calling code needs to handle.

If code performs operations that could put the user at risk if it's called with invalid values, the code should verify the values first and panic if the values are invalid to avoid exposing the code to vulnerabilities.

Functions often have contracts: their behaviour is only guaranteed if the inputs meet particular requirements, it is common to panic if these contracts are violated to indicate a caller-side bug.

- Contracts should be outlined in the API documentation of the function

Rust's type system can do most of the heavy lifting for error checks. The compiler can ensure that values being passed into a function are valid if the function stipulates a particular type. (e.g. a u32 function parameter ensures that the parameter is never negative)

# Creating Custom types for validation

Expanding on the idea of using Rust's type system for error checking, we can make a new type (such as a struct) with built-in validation when creating an instance of that type.

This is useful for values that are used in multiple functions that need the same validation, instead of adding the validation to every function.

Below is an example of a `Guess` type/struct, which contains a number between 1 and 100:

```
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

```

The Guess struct has a field `value`, which stores an i32.

The `new` associated function will create new instances of `Guess`, it takes in a `value` i32 instance, checks if `value` is between 1 and 100, and panics if it's not.

The `value` associated function borrows `self` and returns the `value` field of the borrowed `Guess` instance.

- This type of method is called a _getter_.
- This method is necessary to make the `value` field private, in order to disallow outside modification which can set `value` to an invalid value, and ensure that the `value` field has been checked by the validation in the `new` function.
