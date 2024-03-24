fn main() {
    /*
    // There are two ways to cause a panic

    // Taking an action that causes code to panic (e.g. accessing an array past the end)
    // Or calling the `panic!` macro
    panic!("oopsie woopsie");
    // By default, panics will print a failure message, unwind (clean up the stack from each function called), and quit
    // Via enviroment variable, we can also display the call stack to track down where the code crashed
    */

    //
    // Using a panic backtrace
    //

    // The panic below will come from a library rather than our written code
    let v = vec![1, 2, 3];
    v[99];

    // We can set RUST_BACKTRACE=1 to see a list of all functions that have been called to get to the point of panic
    // The key is to start from the top and read until seeing the files that we wrote
}
