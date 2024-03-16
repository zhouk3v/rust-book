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