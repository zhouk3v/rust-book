fn main() {
    //
    // Variables and Mutability
    //

    // // Variables in rust are immutable by default
    // // Use 'mut' to make them mutable
    
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    //
    // Constants
    //

    // // Constants are always immutable (i.e. can't use mut)
    // // Constants can be declared in any scope
    // // Constants cannot be set a value that can be only computed at runtime

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //
    // Shadowing
    //
    // let x = 5;

    // // This variable shadows the previous declaration (x is now 6)
    // // We are essentially creating a new variable here
    // let x = x + 1;

    // {
    //     // This variable shadows the previous declaration of x (x is now 12)
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // // End of scope, x is now 6 again
    // println!("The value of x is: {x}");

    // // Shadowing can also deal with different types (again, we are essentially creating a new variable)
    // let spaces = "    ";
    // let spaces = spaces.len();

    // // This wont work - mut needs the new value to be the same type
    // let mut spaces = "    ";
    // spaces = spaces.len()
}
