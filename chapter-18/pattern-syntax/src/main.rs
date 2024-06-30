fn main() {
    //
    // Matching literals
    //

    // We can match pattersn against literals directly
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // This syntax is useful when the code needs to take action if it gets a particular concrete value

    //
    // Matching Named Variables
    //

    // Named variables are irrefutable patterns that match any values,
    // However, there is a complication when using named variables in `match` expressions
    // Because `match` starts a new scope,
    // variables declared as part of a pattern inside the `match` expression will shadow those with the same name outside of the `match` construct

    let x = Some(5);
    let y = 10;

    match x {
        // First pattern doesn't match, code continues
        Some(50) => println!("Got 50"),
        // Second pattern introduces a new variable: `y` that will match any value inside a `Some<T>` value.
        // Since we're in a new scope inside the `match` expression, this is a new `y` variable (not the one declared at the beginning)
        // This new `y` binding will match to any value inside a `Some<T>`, thus it will bind to the inner value of the `Some<T>` in `x` (which is 5)
        // The expression for this arm will execute and print `Matched y = 5`
        Some(y) => println!("Matched y = {y}"),
        // If `x` was `None`, it would have matched the underscore
        // We didn't introduce a `x` variable in the pattern of the underscore arm, so `x` is still the outer `x` that hasn't been shadowed
        // In this case, this arm would print out `Default case, x = None`
        _ => println!("Default case, x = {:?}", x),
    }
    // `match` expression scope ends, and so does the scope of the inner `y` in the match expression
    // This line would print out `at the end: x = Some(5), y = 10`
    println!("at the end: x = {:?}, y = {y}", x);

    //
    // Multiple patterns
    //

    // In match expressions, multiple patterns can be matched with the `|` syntax (or operator)
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //
    // Matching Ranges of Values with `..=`
    //

    // `..=` allows matching to an inclusive range of values
    // When a pattern matches any of the values within the given range, that arm will execute
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // ranges are only allowed for numeric or char values
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    //
    // Destructuring to Break Apart Values
    //

    // Patterns can also be used to destructure structs, enums and tuples

    //
    // Destructuring Structs
    //

    // Breaking apart a struct with a `let` statement

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    // The code above creates variables 'a' and 'b' that match the values 'x' and 'y' of the `p` struct

    // Rust has a shorthand to create variables with the same name as the struct fields
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // We can also destructure with literal values as part of the struct pattern
    let p = Point { x: 0, y: 7 };

    match p {
        // First arm will match any `Point` where the `y` field is 0 and creates an `x` variable that can be used in the arm's code
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        // Second arm will match any `Point` where the `x` field is 0 and creates an `y` variable that can be used in the arm's code
        Point { x: 0, y } => println!("On the y axis at {y}"),
        // Third arm matches any other `Point` and creates variables for both the `x` and `y` fields
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})")
        }
    }
    // Remember that a `match` expression stop checking arms once it has found the first matching pattern
    // A `Point {x: 0, y: 0}` will only print `On the x axis at 0`

    //
    // Destructuring Enums
    //
    /*
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // A `match` statement that will destructure each inner value of the enum
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        // For struct-like enum variants, place curly brackets and then list the fields with variables to destructure it
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        // For tuple-like enum variants, the pattern is similar to the pattern used to match tuples
        // The number of variables in the pattern must match the number of elements in the variant we're matching
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
    }
    */

    //
    // Destructuring Nested Structs and Enums
    //

    // Matching can work on nested items too
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        // The first arm matches a `Message::ChangeColor` enum variant that contains a `Color::Rgb` variant,
        // then the pattern binds to the three inner i32 values
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        // The second arm matches a `Message::ChangeColor` enum variant that contains a `Color::Hsv` variant,
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    //
    // Destructuring Structs and Tuples
    //

    // Destructuing nested structs and tuples inside a tuple
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    //
    // Ignoring Values in a Pattern
    //

    // There a few ways to ignore entire values or parts of a value in a pattern:
    // - using the `_` pattern
    // - using the `_` pattern within another pattern
    // - using a name that starts with an underscore
    // - using `..` to ignore remaining parts of a value

    //
    // Ignoring an Entire Value with `_`
    //

    // `_` is a wildcard pattern that will match any value but not bind to the value
    // It is useful for the last arm in a `match` expression, but can be used in any pattern (e.g. function parameters)

    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);

    // When a function parameter is no longer needed, you would change the signature so it doesn't include the unused parameter
    // Ignoring a function parameter can be useful in cases such as implementing a trait
    // where you need a certain type signature but the function body in your implementation doesn't need one of the parameters.
    // This will avoid compiler warnings about unused function parameters

    //
    // Ignoring Parts of a Value with a Nested _
    //

    // We can also use `_` inside another pattern to ignore just part of a value
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        // In the first arm, we don't need to match on or use the value inside either `Some<T>` variant
        // but we do need to test for the case when both `setting_value` and `new_setting_value` are the `Some<T>` variant
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    // We can also use underscores in multiple places within one pattern to ignore particular values
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    //
    // Ignoring an Unused variable by Starting its name with `_`
    //

    // Rust will usually issue a warning for an unused variable, but you can silence this by starting the name of the variable with an underscore

    // No warning will appear for `_x`
    let _x = 5;
    // A warning will appear for `y`
    let y = 10;

    // Note that a variables that starts with `_` still binds the value to the variable, whereas `_` doesn't bind at all

    // This doesn't work
    /*
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }
    // error thrown here, since the `s` value will be moved into `_s`
    println!("{:?}", s);
    */

    // In contrast with `_`
    let s = Some(String::from("Hello!"));

    // The underscore itself doesn't bind to the value
    if let Some(_) = s {
        println!("found a string");
    }
    // Still works, as `s` doesn't get moved into `_`
    println!("{:?}", s);

    //
    // Ignoring Remaining Parts of a value with `..`
    //

    // The `..` pattern ignores any parts of a value that we haven't explicitly matched in the rest of the pattern

    struct Point_3d {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point_3d { x: 0, y: 0, z: 0 };

    match origin {
        // We list the `x` value and then just include the `..` pattern
        Point_3d { x, .. } => println!("x is {}", x),
    }

    // The `..` syntax will expand the as many values as it needs to be
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // The first and last values are matched with `first` and `last`
        // The `..` will match and ignore everything in the middle
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // Using `..` must be unambiguous, if it's unclear which values are intended for matching and which are to be ignored, a compiler error will be thrown

    // This doesn't work
    /*
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // Using `..` twice in a tuple is ambiguous
        // It is impossible for Rust to determine how many values in the tuple to ignore before matching a value with `second`,
        // and then how many futher values to ignore thereafter
        (.., second, ..) => {
            println!("Some numbers: {}", second);
        }
    }
    */

    //
    // Extra Conditionals with Match Guards
    //

    // A match guard is an additional `if` condition, specified after the pattern in a `match` arm,
    // which must also match for that arm to execute
    // They are useful for expressing more complex conditions than a pattern alone allows

    // The condition can use variables created in the pattern
    let num = Some(4);

    match num {
        // In the first arm, `num` matches the `Some(x)` pattern, and then the match guard checks `x % 2 == 0`
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // There is no way to express the `if x % 2 == 0` condition within a pattern, so the match guard allows us to express this logic
    // The downside is that arms with match guards don't count towards exhaustiveness, so we still need the unguarded `Some(x)` arm

    // match guards can also solve the pattern shadowing problem
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // The pattern in the second match arm doesn't introduce a new variable `y` that would shadow the outer `y`, so we can use the outer `y` to match guard
        // We use `Some(n)` instead of `Some(y)` to create a new variable `n` that doesn't shadow anything since there is no outer `n` variable
        // The match guard `if n == y` is not a pattern and therefore doesn't introduce new variables
        // The `y` here is the outer `y`, so we can look for a value that has the same value as the outer `y`
        Some(n) if n == y => println!("Matched, n == {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // We can also use the `|` operator in a match guard to specify multiple patterns
    // The match guard condition will apply to all patterns
    let x = 4;
    let y = false;

    match x {
        // The match condition state that the arm only matches if the value of `x` is equal to 4,5, or 6 and if `y` is true
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // The precedence of a match guard in relation to a pattern behaves like this:
    /**
     *  (4 | 5 | 6) if y => ...
     */

    //
    // `@` bindings
    //

    // The `@` operator allows us to create a variable that holds a value at the same time as we're testing that value for a pattern match

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        // The first arm will capture the value matched in the range with `id_variable` while also testing that the value matched the range pattern
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        // In the second arm that only has a range specified in the pattern,
        // the code associated in the arm doesn't have a variable that contains the actual value of the `id` field
        // The `id` value could have been 10,11, or 12, but the code associated with the pattern doesn't know which it is
        // The pattern code isn't able to use the value from the `id` field, because we didn't save it in a variable
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        // In the last arm, we do have the value available to use in the arm's code in an `id` variable
        // The reason is that we used the struct field shorthand syntax, but we haven't applied any test to the value in the `id` field in this arm
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
