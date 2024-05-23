// The Rc<T> (reference counting) type enables multiple ownership

// The Rc<T> type keeps track of the number of references to a value to determine whether or not the value is still in use.
// If there are zero references to a value, the value can be cleaned up without any references becoming invalid

// We use this type when we want to allocate some data on the heap for multiple parts of our program to read,
// and we can't determine at compile time which part will finish using the data last. (If we did know, we would make that part the owner of the data)
// Example: Graph Nodes and Edges - One node can be owned by multiple edges

//
// Using Rc<T> to Share Data
//

// The original implementation for a Cons List
/*
enum List {
    Cons(i32, Box<List>),
    Nil,
}
*/

/*
use crate::List::{Cons, Nil};

// Trying to create a Cons list with two heads (won't work)

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // a is moved into the Box here
    let b = Cons(3, Box::new(a));
    // Error here - a is used after being moved again
    let c = Cons(4, Box::new(a));
}
*/

// Note that the Cons variant own the data that they hold, (via the usage of Box)
// We could use references instead, but we would need to specify lifetime parameters
// With the usage of lifetime parameters, we would be specifying that every element in the list will live as along as the entire list,
// which may not be true for all cases

// Use Rc<T> in place of Box<T>
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

/*
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // Clone the Rc<T> that a has, which increases the reference count for the Rc<T> by 1
    // The data inside the Rc<T> will not be cleaned up until there are no more references to it
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    // Note that a.clone() also works, but the convention is to use Rc::clone()
    // Rc::clone() does not make a deep copy of the data, rather it just increments the reference count
    // Thus, with this convention, we can tell between the clone() that makes a deep copy (clone() method) and the clone() that makes a shallow copy (Rc::clone())
}
*/

//
// Cloning an Rc<T> Increases the Reference Count
//

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // Rc::strong_count will return the number of references to the Rc<T>
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    // count will down by one when c goes out of scope
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
// Not seen: b and a go out of scope after main, count drops to 0 and the Rc<T> is cleaned up
