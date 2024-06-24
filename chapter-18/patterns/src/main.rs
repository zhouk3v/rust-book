// A pattern consists of some combination of the following:
// - Literals (1, "hello world", true)
// - Destructured arrays, enums, structs, or tuples
// - Variables
// - Wildcards
// - Placeholders

// In the context in which patterns are valid, these components describe the shape of data.
// Our program then matches values against the patterns to determine whether it has the correct shape of data to continue running a piece of code

// Places where patterns are valid

fn main() {
    //
    // `match` arms
    //

    // Patterns are used in the arms of `match` expressions
    // Remember that `match` expressions are defined by the keyword `match`, a value to match on,
    // and one or more match arms that consist of a pattern and an expression to run if the value matches that arm's pattern

    /**
     *
     * match VALUE {
     *      PATTERN => EXPRESSION,
     *      PATTERN => EXPRESSION,
     *      PATTERN => EXPRESSION,
     * }
     */
    // Example: match arm on an Option<i32> value
    let x = Some(42);
    // The patterns in this `match` expression are the `None` and `Some(i)` on the left of each arrow
    match x {
        None => None,
        Some(i) => Some(i + 1),
    };
    // One requirement for `match` expressions is that they need to be exhaustive,
    // i.e. all possibilities for the value in the `match` expression need to be accounted for
    // One way to do so is to have a catch-all pattern for the last arm
    // Example: a variable name matching any value can never fail and thus cover every remaining case

    // The `_` pattern will match anything, but it never binds to a variable. It is useful when wanting to ignore any value not specified in the other match arms

    //
    // Conditional `if let` expressions
    //

    // `if let` expressions are a shorter way to write a `match` expression that only matches one case.
    // Optionally, it can have an `else` block to run if the pattern in the `if let` doesn't match

    // We can mix and match `if let`, `else if`, and `else if let` expressions
    // This is more flexible than a `match` expression, in which we can express only one value to compare with the patterns
    // Rust also doesn't require that the conditions in a series of `if let`, `else if`, `else if let` arms relate to each other

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite_color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // `if let` can also introduce shadowed variables in the same way `match` arms can
    // E.g. the `if let Ok(age) = age` line introduces a new shadowed `age` variable that contains the value inside the `Ok` variant
    // This means that the `if age > 30` is necessary within that `if let` block, we can't combine the two conditions into `if let Ok(age) = age && age > 30`
    // The shadowed `age` isn't valid until the new scope starts within the `if let` block

    // The downside of `if let` expressions is that the compiler will not check for exhaustiveness, as compared to the `match` expression, it does

    //
    // `while let` Conditional Loops
    //

    // The `while let` conditional loop allows a `while` loop to run for as long as a pattern continues to match.

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // pop() takes the last element out of the vector and returns Some(value)
    // If the vector is empty, pop() will return None
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    //
    // `for` loops
    //

    // The value that directly follows the keyword `for` is a pattern.
    // E.g. in `fox x in y`: `x` is a pattern

    // Using a pattern to destructure a tuple as part of the `for` loop
    let v = vec!['a', 'b', 'c'];

    // enumerate() will produce a value from the iterator and the index for that value in a tuple for each iteration
    // When the tuple is matched to the pattern (index, value), index will be the first element of the tuple and value is the second element of the tuple
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    //
    // `let` statements
    //

    // Formally, a `let` statements looks like this:
    /**
     * let PATTERN = EXPRESSION;
     */
    // In statements like `let x = 5`, the variable name is a simple form of a pattern.
    // Rust compares the expression against the pattern and assigns any names it finds.
    // E.g. in `let x = 5`, `x` is a pattern that means "bind what matches here to the variable `x`"
    // Because the name `x` is the whole pattern, this pattern effectively means "bind everything to the variable x, whatever the value is"

    // Another example: destructuring a tuple with `let`
    let (x, y, z) = (1, 2, 3);
    // Rust compares the value `(1,2,3)` to the pattern `(x,y,z)` and sees that the value matches the pattern
    // Rust will then bind `1` to `x`, `2` to `y` and so on.

    // If the number of elements in the pattern doesn't match the number of elements in the tuple, the compiler will throw an error

    //let (x, y) = (1, 2, 3);

    // We could ignore one or more values in the tuple with `_` or `..`
    // If the problem is that we have too many variables in the pattern,
    // the solution is to make the types match by removing variables so the number of variables equals the number of elements in a tuple

    //
    // Function Parameters
    //

    // Function parameters can also be patterns

    // E.g. the `x` part is a pattern
    fn foo(x: i32) {}

    // We can also match a tuple in a function's arguments to the pattern

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    // The values `&(3,5)` match the pattern `&(x, y)`, so `x` = 3, and `y` = 5
    print_coordinates(&point);

    // We can also use patterns in closure parameter lists in the same way as in function parameter lists, because closures are similar to functions
}
