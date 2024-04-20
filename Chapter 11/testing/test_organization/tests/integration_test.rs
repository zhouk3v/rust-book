//
// Integration Tests
//

// Integration tests are entirely external to the library
// They would use the library in the same way any other code would, so they only call functions that are part of the public API
// Their purpose is to test whether many parts of the library work together correctly

//
// The `tests` directory
//

// Create a `tests` directory at the same level as the `src` directory
// Cargo will look for integration tests automatically in this directory
// There can be as many test files as you want

// Each file in the `tests` directory is a seperate crate, so we need to bring our library into each test crate's scope with use
use test_organization;

// Pull in the `common` module to use some setup functions in tests
mod common;

// We don't need to annotate any code in integration test files with `#[cfg(test)]`.
// Cargo compiles everything in the `tests` directory only when `cargo test` is ran
#[test]
fn it_adds_two() {
    // Calling a setup function
    common::setup();
    assert_eq!(4, test_organization::add_two(2));
}

// The output of `cargo test` is split into sections: unit tests, integration tests (one section for each test file), and doc tests
// If any test in a section fails, the following sections will not be run.
// Each integration test file has its own section
// We can run a particular integration test function with the `--test` argument
// E.g. `cargo test --test integration_test`

//
// Integration Tests for Binary Crates
//

// Note that binary crates with only a `src/main.rs` file cannot expose functions to be tested in integration tests
// This is one of the reasons that binary crates usually have a straightforward src/main.rs file that calls logic from the src/lib.rs file
