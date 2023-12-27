/*
fn main() {
    let number = 7;

    // condition is an expression that must return a boolean - Rust does not auto convert non boolean types into boolean
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
*/

//
// else if
//
/* 
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
*/

//
// Using if in a let statement
//


fn main() {
    let condition = true;
    // if is an expression and thus, returns a value (in this case, the expression at the end of whatever scope is executed)
    // Note that the types that each block returns need to match
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

