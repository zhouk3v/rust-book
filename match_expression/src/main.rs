#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    // 'match' will compare a value against a series of patterns and then execute code based on which pattern matches

    // The condition is a expression, which can be anything (in this case, it's the 'coin' parameter)
    // The body of the match expression consists of 'arms'
    // Each arm has a pattern and some code, which are seperated by '=>'
    // When the match expression executes, it compares the resulted value of the condition expression against the pattern in each arm, in order.
    // If the pattern matches the value, the code associated with that pattern executes, if not, the execution continues to the next arm
    // The code in each arm is an expression, and the resultant value of the expression in the matching arm is returned for the entire match expression
    // The arms' pattern must cover all possibilities
    match coin {
        // We can use curly brackets to include additional code along with an expression to return
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Match expressions can extract values associated with a matching enum variant
        // In this example, the 'state' variable binds to the value inside the Quarter enum value
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

//
// Matching with Option<T>
//

// Example of using the match expression to deal with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));
    let dice_roll = 9;


    //
    // Catch-all patterns and the _ Placeholder
    //
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // This is a catch all arm, which will match all values not specifically listed
        // We have to put the catch all arm last since the pattersn are evaluated in order
        other => move_player(other),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // We can use _ as a pattern if we don't need the catch all value
        _ => reroll(),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // We can return a unit value to express that nothing happens
        _ => (),
    }

    //
    // How Matches Interact with ownership
    //

    let opt: Option<String> = Some(String::from("Hello world"));

    // If we use the _ placeholder, the data will not be moved, so we will still be able to use it after
    match opt {
        Some(_) => println!("Some!"),
        None => println!("None!"),
    };
    println!("{:?}", opt);

    // If we use a variable to bind to the value associated with the enum variant, then ownership is transferred from the passed in enum variant instance
    match opt {
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };

    // This won't work, the value for opt was moved in the match statement above
    // println!("{:?}", opt);

    let opt: Option<String> = Some(String::from("Hello world"));

    // The idiomatic way is to use a reference for the match expression
    // Rust will push down the reference from the outer enum (&Option<String>) to the inner field (&String), so the code in the branch will receive a reference
    match &opt {
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };

    // opt can still be used here, as the match statement only borrowed the value
    println!("{:?}", opt);
}

// Functions for the Catch-all patterns and the _ Placeholder example, implementation doesn't matter
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
