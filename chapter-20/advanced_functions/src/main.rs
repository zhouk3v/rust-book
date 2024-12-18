//
// Function Pointers
//

// We can pass regular functions to functions, which is useful for passing an already defined function rather than defining a new closure
// Function coerce to the type `fn` (not to be confused with the `Fn` closure trait), which is known as a "function pointer"
// Function pointers will allow the usage of functions as arguments to other functions

// The syntax for specifying a function pointer parameter is similar to that of closure parameters
fn add_one(x: i32) -> i32 {
    x + 1
}

// The parameter `f` is an `fn` that takes one parameter of type `i32` and returns an `i32`
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    // Call f() in the body of do_twice()
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop,
}

fn main() {
    // Pass the function add_one() as the first argument to do_twice()
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
    // Unlike closures, `fn` is a type rather than a trait, so we specify `fn` as the parameter type directly,
    // rather than declaring a generic type parameter with one of the `Fn` traits as a trait bound

    // Note that function pointers implement all three closure traits (`Fn`, `FnMut`, `FnOnce`)
    // Thus, a function pointer can be used as an argument for a function that expects a closure
    // It's best to write functions using a generic type and one the closure traits so your functions can accept either functions or closures

    // The exception to the above is when interfacing with external code that doesn't have closures (e.g. C)

    // Example of using either a closure defined inline or a named function: the map() method

    // Using a closure with map()
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    // Using a named function
    let list_of_numbers = vec![1, 2, 3];
    // Note the usage of fully qualified syntax since there are multiple to_string() functions
    // Here, we're using the to_string() function defined in the `ToString` trait, which is implemented for any type that implements `Display`
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    // Note that in enums, each enum variant defined also becomes an initializer function
    // We can use these initializer functions as function pointers that implement the closure traits
    // So we can specify the initializer functions as arguments for methods that take closures
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

//
// Returning Closures
//

// Closures are represented by traits, so we can't return closures directly
// Also, closures don't have a concrete type that is returnable

// This won't work
/*
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
*/

// The problem is that Rust doesn't know how much space it will need to store the closure
// To fix this, use a trait object
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
