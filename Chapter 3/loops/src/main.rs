//
// Repeating code with loop
//

/*
fn main() {
    loop {
        println!("again!");
    }
}
*/

//
// Returning values from loops
//

/*
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // Put a value to the right of the break keyword to return a value from the loop
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
*/

//
// Loop labels to Disambiguate Between Multiple Loops
//

/* 
fn main() {
    let mut count = 0;
    // We can label loops to distinguish between them for break and continue statements
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // this break will affect the inner most loop (line 44)
                break;
            }
            if count == 2 {
                // this break will affect the counting up loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
*/

//
// Conditional loops with while
//

/*
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!")
}
*/

//
// Looping through a collection with for
//

/*
fn main() {
    let a = [10,20,30,40,50];

    for element in a {
        println!("the value is: {element}")
    }
}
*/

// using a range with a for loop

fn main() {
    // .rev() will reverse the range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

