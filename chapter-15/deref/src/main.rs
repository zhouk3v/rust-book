// Implementing the `Deref` trait allows customization of the behaviour of the dereference operator (*)
// By implementing `Deref` in a way that a smart pointer can be treated like a regular reference,
// code can be written that operates on references and used on smart pointers too.

/*
fn main() {
    //
    // Following the Pointer to the Value
    //

    // Regular references are a type of pointer (memory address - printing out with the :p formatter will display the actual address)
    // One way to think of a pointer is as an arrow to a value stored somewhere else.

    let x = 5;
    // A reference to an i32 value
    let y = &x;

    assert_eq!(5, x);
    // Using the Deref operator to follow the reference to the value
    assert_eq!(5, *y);

    // This won't work
    // assert_eq!(5, y);
    // Comparing a number and a reference to a number won't work, since they are different types (remember that references are pointers/addresses)

    //
    // Using Box<T> like a reference
    //

    let x = 5;
    // y is an instance of a Box<T> pointing to a copied value of x (note that i32 values are copied rather than moved)
    let y = Box::new(x);

    assert_eq!(5, x);
    // The Deref operator can also follow the pointer of the Box<T> in the same way for a reference.
    assert_eq!(5, *y);
}
*/

//
// Defining Our Own Smart Pointer
//

// The Box<T> type is defined by a tuple struct with one element
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

//
// Treating a Type Like a Reference by Implementing the `Deref` Trait
//

// The `Deref` standard library trait requires the implementation of one method: the `deref` method
// The `deref` method borrows `self`, and returns a reference to the inner data.

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    // Associated Type (more on this later)
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // `deref` returns a reference to the value we want to access with the * operator
        // This method gives the compiler the ability to take a value of any type which implements `Deref` and call this method to get a & reference that can be dereferenced
        &self.0
    }
}

/*
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
*/

// Behind the scenes, rust actually ran `*(y.deref())`
// Rust subsitutes the `*` operator with a call to the `deref` method and then a plain dereference so we don't have to think about calling deref()

// `deref` returns a reference (and why * is still necessary) is because of the ownership rules:
// If the `deref` method returned the value directly instead of a reference to the value, the value would be moved out of `self`
// We do not want to take ownership of the inner value inside MyBox<T> in most cases where we use the dereference operator

// Note that the * operator is replaced with the snippet above only once, so it does not recurse infinitely

//
// Implicit Deref Coercions with Functions and Methods
//

// Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type
// Example: Deref coercion converts &String to &str, since String implements the Deref trait such that it returns &str
// This a convenience that Rust performs on arguments to functions and methods and only works on types that implement Deref
// It happens automatically when we pass a reference to a particular type's value as an argument to a function/method that doesn't match the parameter type
// A sequence of deref calls converts the type we provided into the type the parameter needs

// Example of Deref coercion

// A function that takes a string slice (&str)
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    // Calling hello() with a reference to a MyBox instance
    // Rust will turn &MyBox<String> into a &String by calling MyBox's implementation of deref
    // Rust will then turn the &String into a &str by calling String's implementation of deref
    // At compile time (meaning no runtime penalty), Rust will analyze the types and use Deref::deref as many times as necessary to get a reference to match the parameter type
    hello(&m);

    // Calling hello() without Deref coercion
    // (*m) derefs MyBox<String> into a String
    // & creates a &String reference
    // [..] turns the &String into a string slice
    hello(&(*m)[..]);
}

//
// How Deref Coercion Interacts with Mutability
//

// You can use the DerefMut trait to override the * operator on mutable references

// Rust does deref coercion when it finds types and trait implementations in three cases

// - From &T to &U when: Deref<Target = U>
// - From &mut T to &mut U when T: DerefMut<Target = U>
// - From &mut T to &U when T: Deref<Target=U>

// For the third case, Rust will coerce a mutable reference to an immutable one, but the reverse is not possible -> immutable references will never coerce to mutable references
// Remember that the borrowing rules only allow one mutable reference at a time
// Coverting an immutable reference to a mutable reference would require that the initial immutable reference to be the only reference to that data,
// but this is not a guarantee
