//
// Implementing the trait (cont)
//

// Someone using the library can define a custom type (`SelectBox`), which will implement the `Draw` trait
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a SelectBox");
    }
}

//
// Using the trait
//

/*
use gui::{Button, Screen};

fn main() {
    // Creating a `Screen` instance
    let screen = Screen {
        // Adding a `SelectBox` and a `Button` to components
        // Note the usage of Box<T> to become a trait object
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    // Calling the run() method, which will call draw() on each of the components
    screen.run();
}
*/

// Note that the library did not have the `SelectBox` type originally (it was defined by the user),
// but the `Screen` instance was able to operate on the new type regardless because `SelectBox` implements the `Draw` trait

// The concept of only being concerned only with the messages a value responds to rather than the value's concrete type,
// is similar to the concept of `duck typing` in dynamically typed languages
// In the implementation of run() in `Screen`, run() doesn't need to know what the concrete type of each component is, it just calls the draw() method on the component.
// By specifying Box<dyn Draw> as the type of the values in the `components` vector, we've defined `Screen` to need values that we can call the draw() method on

// The advantage of trait objects and Rust's type system to write code similar to using duck typing
// is that we don't need to check whether a value implements a particular method at runtime
// or worry about getting errors if a value doesn't implement a method but we call it anyway.
// The compiler will catch these errors

// Eg. trying to create a `Screen` with a `String` as a component

/*
use gui::Screen;

fn main() {
    let screen = Screen {
        // A compile-time error is thrown here, as String does not implement the `Draw` trait
        components: vec![Box::new(String::from("Hi"))];
    };

    screen.run();
}
*/

//
// Trait Objects and Type Inference
//

// A downside of using trait objects is how they interact with type inference

// Eg: Vec<T> type inference
fn main() {
    // An empty vector will cause a type inference error:
    /*
    let v = vec![];
    */
    // But adding an element will allow Rust to infer the type of the vector
    let v = vec!["Hello World"]; // Rust will infer the type Vec<&str>

    // Trying to declare a vector of trait objects
    // This won't work, a compile-time error is thrown for mismatched types
    /*
    let components = vec![
        Box::new(SelectBox {
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ],
        }),
        Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        }),
    ];
     */
    // We can fix the above with an explicit cast on any element of the vector
    // NOTE (not in the book) - a trait object cannot be "downcasted" to a more concrete type
    let components = vec![
        Box::new(SelectBox {
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ],
        }) as Box<dyn Draw>,
        Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        }),
    ];
    // or with a type annotation
    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(SelectBox {
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ],
        }),
        Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        }),
    ];
    let screen = Screen { components };
    screen.run();
}

//
// Trait Objects Perform Dynamic Dispatch
//

// Remember that when using trait bounds on generics, the compiler will generate nongeneric implementations of functions and methods for each concrete type used
// The code that results from this monomorphization is doing `static dispatch`

// When using trait objects, Rust must use dynamic dispatch, where the compiler can't tell at compile time which method to call.
// In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.
// At runtime, Rust uses the poiinters inside the trait object to know which method to call
// This lookup incurs a runtime cost
// Dynamic dispatch also prevents the compiler from choosing to inline a method's code, which in turn prevents some optimizations
// In return, you do get some flexibility in the code
