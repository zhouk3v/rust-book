// Rust divides tests into two categories:
// - Unit tests: small, focused tests that deal with one module in isolation at a time, and can test private interfaces
// - Integration tests: external to the library, deal with only the public interface and potentially exercising multiple modules per test

//
// Unit Tests
//

/*
// Purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn't working as expected
// Unit tests are put in the src directory in each file with the code that they're testing
// The convention is to create a module named `tests`` in each file to contain test functions and to annotate it with `cfg(test)`

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

//
// The Tests Module and `#[cfg(test)]`
//

// The `#[cfg(test)] annotation tells Rust to compile and run the test code only in `cargo test`
// Note that a test module is auto-generated in lib crates (from cargo new --lib)
// `cfg` stands for configuration and tells Rust that the following item should only be included given a certain configuration option (in this case, `test`)
// By using this attribute, cargo will only compile the test code only with `cargo test`, this also includes helper functions in the test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/

//
// Testing Private Functions
//

// Rust's privacy rules do allow you to test private functions

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// Note that the `internal_adder` function is not marked as `pub`
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // The `internal_adder` function is brought in with the glob operator here
    // Remember that items in child modules can use the items in their ancestor modules, regardless if they're private or public
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
