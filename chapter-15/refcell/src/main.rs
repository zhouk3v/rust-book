// Interior mutability is a design pattern that allows the mutation of data even when there are immutable references to that data
// This pattern uses `unsafe` blocks inside a data structure to bend Rust's borrowing rules.

// We can use types that use the interior mutability pattern only when we can ensure that the borrowing rules will be followed at runtime,
// even though the compiler can't guarantee that.
// The `unsafe` blocks are then wrapped in a safe API, and the outer type is still immutable

//
// Enforcing Borrowing Rules at Runtime with RefCell<T>
//

// RefCell<T> represents single ownership over the data it holds.

// With references and Box<T> - the borrowing rules are enforced at compile time
// If the rules are broken when using references/Box<T>, a compiler error is thrown
// With RefCell<T> - the borrowing rules are enforced at runtime
// If the rules are broken when using RefCell<T>, the program will panic and exit

// The advantage of checking borrowing rules at compile time means no impact on runtime performance,
// since the analysis is completed beforehand in a one-time procedure. Thus it is the best choice in the majority of cases

// The advantage of checking the borrowing rules at runtime is that it allows certain memory-safe scenarios that are usually disallowed by the compile-time checks
// Remember that static analysis is inherently conservative, so if Rust can't be sure if the code compiles with the ownership rules, it might reject a correct program

// The RefCell<T> type is useful when the code follows the borrowing rules but the compiler is unable to understand and guarantee that.

// Note that RefCell<T> is not thread-safe

// Recap of Box<T>, Rc<T> and RefCell<T>
// - Rc<T> enables multiple owners of the same data,
//  - Box<T> and RefCell<T> have single owners
// - Box<T> allows immutable or mutable borrow checks at compile time
//  - Rc<T> only allows immutable borrow checks at compile time
//  - RefCell<T> allows immutable or mutable borrow checks at runtime
// - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable

//
// Interior Mutability: A Mutable Borrow to an Immutable Value
//

// In the borrowing rules, if you have an immutable value, you can't borrow it mutably
/*
fn main() {
    let x = 5;
    // Doesn't work, throws a compiler error
    let y = &mut x;
}
*/

// However, it is useful for a value to mutate itself in its methods, but appear immutable to other code.
// Code outside the value's methods would not be able to mutate the value
// RefCell<T> is one way to provide that functionality

//
// Having mutliple owners of mutable data by combing Rc<T> and RefCell<T>
//

// Remember that Rc<T> allows multiple owners to some data, but only gives immutable access to that data
// If an Rc<T> holds a RefCell<T>, you get a value that can have multiple owners and can be mutated

// Eg: Cons list where you can mutable the value of the nodes
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Create `value` which is an i32 wrapped in an RefCell<T> that is wrapped in a Rc<T>
    let value = Rc::new(RefCell::new(5));

    // Create a Cons variant that holds `value`
    // We need to clone `value` so both `a` and `value` have ownership of the inner 5 value rather than transferring ownership from `value` to `a` or having `a` borrow from `value`
    // Wrap the Cons in `a` in an Rc<T> so when we create more lists, they can refer to `a` at the same time
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // Add 10 to the value in `value` by calling borrow_mut(), which will auto dereference the Rc<T> to the inner RefCell<T> value
    // The borrow_mut() method retuns a RefMut<T> smart pointer, and we use the dereference operator on it and change the inner value
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// Note that RefCell<T> is not thread-safe (use Mutex<T> for that)
