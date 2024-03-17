//
// Defining Modules to Control Scope and Privacy - Paths for Referring to an Item in the Module Tree
//

/* 
mod front_of_house {
    // Need to make the hosting module public (modules are private by default) to allow ancestor modules to refer to it (note that siblings are NOT ancestors)
    // Items in a parent module can't use the private itesm inside child modules, but items in child modules can use the items in their ancestor modules
    // This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they are defined
    // This way makes it easier to tell which parts of the inner code you can change without breaking outer code
    pub mod hosting {
        // Also need to make seperate parts of the module public to allow ancestor modules to access the public module's inner code
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path - the full path that starts from a crate root, begins with the crate name, 
    // or for code from the current crate, it starts with the literal 'crate'. (which we use here, as we are already in the crate)
    // Note that we don't need to make front_of_house public to use it in eat_at_restaurant, as front_of_house and eat_at_restaurant are siblings
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path - starts from the current module, and uses self, super or (in this case) an identifier in the current module 
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // The fix_incorrect_order is in the back_of_house module, but we can use super() to refer to the parent module(in this case, crate) and gain access to deliver_order()
        // Think of super() as the .. command in bash
        super::deliver_order();
    }

    fn cook_order() {}


}

mod back_of_house {
    // If we make a struct public, we still need to make each field public on a case-by-case basis
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // Since back_of_house::Breakfast has a private field, the struct needs to provide a public associated function that constructs an instance of Breakfast
    // We wouldn't be able to create an instance of Breakfast in eat_at_restaurant, because we would be able to set the seasonal_fruit value
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // We can read and write to the toast property since it's marked public
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // But we can't read or write to the seasonal_fruit property
    // println!("I'd like {} please", meal.seasonal_fruit);
    // meal.seasonal_fruit = String::from("blueberries");
}

mod back_of_house {
    // If we make an enum public, we make all variants public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
*/

//
// Bringing paths into scope with the use keyword
//

/* 
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Create a shortcut in a path with the use keyword
// Similar to creating a symbolic link in a filesystem
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // hosting is now a valid name in the crate root
    hosting::add_to_waitlist();
}

// Note that use only applies in the scope that it's in
// The previous use only applies in the crate scope, and so is invalid in the customer scope below
// To fix it, create a new use statement in customer, or use 'super' to go up to the crate scope
mod customer {

    // Creating a new use statement
    //use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        //hosting::add_to_waitlist();

        // Using super to go up to the crate scope
        super::hosting::add_to_waitlist();
    }
}
*/

//
// Creating idiomatic use paths
//

/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// It is unidiomatic to bring an entire function in with use
// We need to specify the parent module for the function
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // It is not clear here that the add_to_waitlist function is not locally defined
    add_to_waitlist();
}
 

// We need to use the full path when bringing in structs, enum, and other items with use.
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// Exception: bringing in two items with the same name into scope

// Both the fmt and io modules have a Result object
use std::fmt;
use std::io;

fn function1() -> fmt::Result {

}

fn function2() -> io::Result {

}
*/

//
// Providing New Names with the 'as' keyword
//

// We can use 'as' to specify an alias for a type from a module

/*
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {

}

fn function2() -> IoResult {

}
 */

//
// Re-exporting Names with pub use
//

/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Using 'pub' in front of 'use' will allow external code to use the shortcut too
// This is called re-exporting
// If the use shortcut is not public, external code will need to use 'restaurant::front_of_house::hosting' instead
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
*/

//
// Using External Packages
//

// To use external packages, add it to Cargo.toml

// Then bring it into the scope with 'use'
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}

// Note that the std library is a crate that is external, but the library is packaged with the Rust language, so we don't need to import it into Cargo.toml
use std::collections::HashMap;


//
// Using Nested Paths to Clean Up Large use Lists
//

// We can group use statements from the same module with a nested path
use std::{cmp::Ordering, io};

// Nested paths can be used at any level
// The below brings in std::io and std::io::Write into the scope
use std::io::{self, Write};

//
// The Glob operator
//

// The *glob operator brings in all public items in a module into the scope
use std::collections::*;




