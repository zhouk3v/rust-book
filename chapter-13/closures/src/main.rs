// Closures are anonymous functions that can be saved as a variable or passed into a function
// The closure can be created in one place and then called elsewhere
// Closures can capture values from the scope in which they're defined in

//
// Capturing the Environment with Closures
//

use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // The unwrap_or_else() function takes a closure as an argument
        // The closure doesn't have any arguments and returns a value of the same type in the Option<T>
        // If the Option<T> is None, unwrap_or_else() will call the closure and returns the value returned by the closure
        // The closure captures an immutable reference to the `self` `Inventory` instance (argument in giveaway()) and passes it with the code we specify to unwrap_or_else()
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    //
    // Closure Type Inference and Annotation
    //

    // Closures don't usually require type annotations to the parameters or return values
    // This is because they aren't an exposed interface,
    // they are stored in variables and used without naming them and exposing them to the users of the library
    // Closures are typically short and relevant only in a narrow context
    // With the limited context, the compiler can infer the types of the parameters and return value
    // We can still add type annotations however

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // Comparision of function and closure syntax

    // function definition
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    // Fully annotated closure
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // Closure without type annotations
    /*
    let add_one_v3 = |x| {x + 1};
    */
    // Closure without brackets
    /*
    let add_one_v4 = |x| x + 1;
    */

    // For closures, The compiler will infer one concrete type for each of their parameters and their return value
    // The first type inferred for the closure will be locked into the closure and can't be changed

    // Closure with inferred types
    let example_closure = |x| x;

    // In example_closure, x will now be inferred as a string
    let s = example_closure(String::from("hello"));
    // Won't work, as x in example_closure() is already inferred as a string before
    // let n = example_closure(5);

    //
    // Capturing References or Moving Ownership
    //

    // Closures can capture values from their enviroment (i.e. the variables on the stack when the closure was created) in three ways:
    // - borrowing immutably
    // - borrowing mutably
    // - taking ownership
    // The closure will determine what way to use based on what does the body of the function do with the captured values

    // Closure that captures an immutable reference
    // Note that the `list` variable is still able to be read with immutable references before the closure definition
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    // The `list` variable is still able to be read with immutable references between the closure definition and calling,
    println!("Before calling closure: {:?}", list);
    // Example that a variable can bind to a closure definition, so we call the closure using the variable name and parentheses
    only_borrows();
    // The `list` variable is still able to be read with immutable references after the closure call
    println!("After calling closure: {:?}", list);

    // Closure that captures a mutable reference
    // We can read the list before the closure definition,
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // We can't immutably borrow between the closure definition and call, since the closure captures a mutable reference for list, which is still active
    //println!("Before calling closure: {:?}", list);

    borrows_mutably();
    // We can read the list again after the closure call, since the mutable borrow from the closure is complete
    println!("After calling closure: {:?}", list);

    // Use the 'move' keyword before the parameter list to force the closure to take ownershup of the values it uses
    // This is useful to move data into a new thread so that the new thread owns it
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // We move the data into the new thread since we don't know if the main thread or the new thread will finish first
    // If the closure in the new thread only captures a immutable reference to the list,
    // there is a chance that the main thread finishes first and drops the data for the list,
    // rendering the reference in the new thread to be invalid
    thread::spawn(move || println!("From thread {:?}", list))
        .join()
        .unwrap();

    //
    // Moving Captured Values Out of Closures and the `Fn` traits
    //

    // The code in the body of the closure defines what happens to the references or values when the closure is evaluated later

    // A closure body can do any of the following:
    // - Move a captured value out of the closure
    //   - Note: Moving a captured value here means transferring ownership of the value to another function,
    //     it does not necessarily mean moving the value back into the environment!
    // - Mutate the captured value
    // - Neither move nor mutate the value
    // - Capture nothing from the enviroment to begin with

    // The way closure captures and handles values from the enviroment affects which `Fn` traits the closure implements
    // Functions and structs also use these `Fn` traits to define what closures they can use
    // Closures automatically implement one, two, or all three of the following `Fn` traits, in an additive fashion, depending on the closure body

    // 1. `FnOnce` applies to closures that can be called once.
    // - All closures implement this trait.
    // - A closure that moves captured values out of its body (i.e. by transferring ownership to another function) will only implement this trait,
    // since it can only be called once

    // 2. `FnMut` applies to closures that don't move captured values out of their body, but might mutate the captured values.
    // - These closures can be called more than once

    // 3. `Fn` applies to closures that don't move captured values out of their body and that don't mutate captured values,
    // and capture nothing from the enviroment
    // - These closures can be called more than once, without mutating the enviroment, which is important for closures that called multiple times concurrently

    // Example: the unwrap_or_else() method on `Option<T>` uses a closure that implements `FnOnce`
    // Note, that the closure takes no arguments, and returns a T
    /*
    impl<T> Option<T> {
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T
        {
            match self {
                Some(x) => x,
                None => f(),
            }
        }
    }
    */

    // This means that unwrap_or_else() is only going to call the closure at most one time
    // Since all closures implement `FnOnce`, unwrap_or_else() accepts the most different kinds of closures
    // (provided that they take in no arguments and return an instance of type T)

    // Note: Functions can implement all three of the Fn traits too.
    // If what we want to do doesn't require capturing a value from the enviroment,
    // then we can use the name of a function rather than a closure when we need something that implements one of the `Fn` traits

    // Another example: The sort_by_key() method on slices takes in a closure that implements `FnMut`

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // sort_by_key() takes in a `FnMut` closure is that it calls the closure multiple times: once for each item in the slice.
    // The closure `|r| r.width` doesn't capture, mutate or move out anything from its enviroment, so it meets the trait bound requirements

    // An example of counting the number of sorting operations, which doesn't work
    /*
    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });

    println!("{:#?}", list);
    */
    // The closure in sort_by_key() captures `value`,
    // then moves `value` out of the closure by transferring ownership of value to the `sort_operations` vector (rendering `value` as invalid)
    // This closure can only be called once, calling it a second time wouldn't work because `value` is no longer in the enviroment

    // A fixed example of counting the number of sorting operations

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);

    //
    // Closures Must Name Captured Lifetimes
    //

    // Example of a function that returns a closure which doesn't work
    /*
    fn make_a_cloner(s_ref: &str) -> impl Fn() -> String {
        // Note that the returned closure captures a reference
        move || s_ref.to_string()
    }
    // This won't compile since there might be a case where the closure is created by make_a_cloner(),
    // then the value behind s_ref is dropped,
    // and then the closure from make_a_cloner() is called (on a now invalid reference)
    let s_own = String::from("Hello world");
    let cloner = make_a_cloner(&s_own);
    drop(s_own);
    cloner();
    */

    // We need to tell Rust that the closure returned from make_a_cloner must not live longer than s_ref
    // This can be done with a lifetime parameter

    fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
        // Note: the `move` here means "take ownership of the reference"
        move || s_ref.to_string()
    }
    // The changes say:
    // - s_ref is a string reference that lives for 'a
    // - Adding `+ 'a` to the return type's trait bounds indicates that the closure must live no longer than 'a

    // Rust will recognize that as long as the make_a_cloner closure still exists (before it's called), `s_own` cannot be dropped
    /*
    let s_own = String::from("Hello world");
    let cloner = make_a_cloner(&s_own);
    drop(s_own);
    cloner();
    */

    // We can use lifetime elision rules to make it cleaner:
    fn make_a_cloner_v2(s_ref: &str) -> impl Fn() -> String + '_ {
        move || s_ref.to_string()
    }
    // We can replace the `'a` generic lifetime over the whole function with the `'_` indicator on the return type,
    // to show that the returned closure depends on some lifetime
}
