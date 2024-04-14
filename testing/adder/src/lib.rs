// Tests are Rust functions that verify non-test code
// They typically perform three actions
// 1. Set up any needed data or state
// 2. Run the code you want to test
// 3. Assert the result are what you expect

// Some functions and methods to test
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    // String::from("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

// Rust will auto generate a test module with a test function for every new library project
#[cfg(test)]
mod tests {
    // Note that the tests module is an inner module
    // We nned to bring the code under test in the outer module into the scope of the inner tests module
    // We use a glob here so anything we define in the outer module is available to the tests module
    use super::*;

    // The simplest test is a function that is annotated with the test attribute
    // To change a function into a test function, add `#[test]` on the line before fn
    // When running tests with the cargo test command, Rust builds a test runner binary that runs the annotated functions,
    // and reports on whether each test function passes or fails
    #[test]
    fn exploration() {
        let result = add(2, 2);
        // This test uses the assert_eq! macro to assert that `result` equals 4
        assert_eq!(result, 4);
    }

    // Note that we can have non-test functions in the tests module which setup common scenarios or perform common operations
    // So we need to always indicate which functions are tests

    // A test that always fails
    // Tests fail when something in the test function panics
    // Each test is run in a new thread, so when the main thread sees that a test thread has died, the test is marked as failed
    // The simplest way to fail a test is to call the panic! macro
    /*
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
     */

    //
    // Checking Results with the `assert!` macro
    //

    // The assert! macro takes in an argument that evaluates to a Boolean,
    // If the value is `true`, nothing happens,
    // If the value is `false`, the `assert!` macro calls `panic!`

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // feed the result of can_hold() directly into assert! since it returns a boolean
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // Since the result of the can_hold function here is `false`, we need to negate before we pass it into the assert! macro
        assert!(!smaller.can_hold(&larger));
    }

    //
    // Testing Equality with the `assert_eq!` and `assert_ne!` Macros
    //

    // The `assert_eq!` and `assert_ne!` macros compare two arguments for equality or inequality, respectively
    // They'll also print the two values if the assertion fails, to show why the test failed
    // Compare this to the `assert!` macro, which only indicates that it got a `false` value

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // Note that the arguments to equality assertion functions are called `left` and `right` in Rust, and the order of the value that we expect and the actual value don't matter

    // The `assert_ne!` macro will pass if the two values we give it are not equal and fail if they're equal
    // This macro is useful for cases where the value is indeterminate, but we know what the values definately shouldn't be.
    // Eg. a random number generator that only outputs positive numbers

    // The `assert_eq!` and `assert_ne!` macros use the `==` and `!=` operators respectively
    // When the assertion fail, these macros print their arguments using debug formatting,
    // This means that the values being compared must implement the `PartialEq` and `Debug` traits
    // All primitive types and most standard library types implement these traits
    // For user-defined structs and enums, they need to implement both traits
    // Note that since these traits are derivable traits, it is usually done with the `#[derive(PartialEq, Debug)]` annotation

    //
    // Adding Custom Failure Messages
    //

    // The `assert!`, `assert_eq!` and `assert_ne!` macros take in optional arguments for custom message
    // These arguments are passed into a `format!` macro, so you are able to pass in a string with {} placeholders, along with values to go into the placeholders
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contaion name, value was {}",
            result
        );
    }

    //
    // Checking for Panics with should_panic
    //

    // It is also important to check that code handles error conditions
    // Add the attribute `should_panic` to let the test pass if the code inside the test function panics
    // The test will fail if the code doesn't panic
    #[test]
    // We can add an optional `expected` parameter to the should_panic attribute to verify that the panic message contains the provided text
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    //
    // Using Result<T, E> in Tests
    //

    // We can also write tests that use `Result<T, E>`
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            // Return Ok(()) if the test passes
            Ok(())
        } else {
            // Return Err<String> if the test fails
            Err(String::from("two plus two does not equal four"))
        }
    }

    // Writing test that return `Result<T,E>` enables the usage of the ? operator in the body of tests,
    // which can be a convenient way to write tests that should fail if any operation within them returns an `Err` variant
    // - Remember that the ? operator is used in functions that return `Result<T, E>` and returns the value inside Ok<T>, or causes the whole function to return Err

    // However, tests that return `Result<T,E>` are unable to use the #[should_panic] annotation
    // To assert that an operation returns a `Err` variant, use `assert!(value.is_err())`
}
