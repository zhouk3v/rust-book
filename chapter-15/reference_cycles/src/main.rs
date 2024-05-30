// Rust's memory safe guarantees make it difficult, but not impossible to accidentally create memory leaks
// Note that memory leaks are memory safe in Rust (but will cause performance problems and crashes)
// Rust allows memory leaks by using Rc<T> and RefCell<T>, it is possible to create references where items refer to each other in a cycle
// The reference count of each item in the cycle will never reach 0, and the values will never be dropped

//
// Creating a Reference Cycle
//

/*
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    // The second element in the Cons variant is a RefCell<Rc<List>>, which allows us to modify the List value that the Cons variant is pointing to
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // The tail() method makes it convenient to access the second item of a Cons variant
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}


fn main() {
    // Create a List in `a`
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // Create a list in `b` which points to the list in `a`
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // Modify `a` to point to `b` instead of Nil to create a cycle
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    // After main() Rust drops the variable `b`, which decreases the reference count of the `b` Rc<List> instance from 2 to 1.
    // Note that the memory that the `b` Rc<List> has on the heap won't be dropped at this point, since the reference count is not 0. (The 'a' Rc<List> instance still refers to it)
    // Rust then drops the variable 'a', which decreases the reference count of the 'a' Rc<List> instance from 2 to 1.
    // Again, the heap memory for the `a` Rc<List> won't be dropped, since the reference count is not 0. (The 'b' Rc<List> instance still refers to it)
}
*/

// If a more complex program allocated lots of memory in a cycle, and held it for a long itme, the program would use more memory than usual, and might crash the system
// Creating reference cycle is not easily done, but not impossible
// Be careful with nested Rc<T> in RefCell<T> instances, or similar nested combinations of types with interior mutability and reference counting.
// Rust cannot catch cycles, so leverage automated tests, code reviews and other software development practices to minimize the chance of cycles happening

// Another solution to avoid cycles is to reorganize data structure so that some reference express ownership and some references don't.
// As a result, you can have cycles made up of some ownerships relationships and some non-ownership relationships, and only the ownerships relationships affect whether or not the value can be dropped
// Note that this is not possible in all cases, (e.g. the Cons variant needs to own their list, so reorganizing the data structure isn't possible).

//
// Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>
//

// A weak reference to the value within an Rc<T> instance by calling `Rc::downgrade()` and passing a reference to the Rc<T>
// Weak references don't express an ownership relationship, and their count doesn't affect when an Rc<T> instance is cleaned up.
// They won't cause a reference cycle because any cycle involving some weak references will be broken once the strong reference count of values involved is 0.

// When calling Rc::downgrade(), it returns a smart pointer of type Weak<T>.
// Instead of increasing the `strong_count` in the Rc<T> instance by 1, calling Rc::downgrade increases the `weak_count` by 1.
// The Rc<T> type uses `weak_count` to track how many Weak<T> references exist.
// The difference is that `weak_count` doesn;t need to be 0 for the Rc<T> instance to be cleaned up

// Because the value that Weak<T> references might have been dropped, we need to check if the value still exists before doing anything with it
// To check, call the `upgrade()` method on a Weak<T> instance, which will return an Option<Rc<T>>.
// The Option<Rc<T>> will be `Some` if the Rc<T> value has not been dropped yet, and a result of `None` if the Rc<T> value has been dropped.
// Rust will ensure that the `Some` case and the `None` case are handled, and there won't be an invalid pointer

//
// Creating a Tree Data Structure: a Node with Child nodes
//

/*
// A `Node` struct that holds its own i32 value and references to its children `Node` values
#[derive(Debug)]
struct Node {
    value: i32,
    // We want a `Node` to own its children, and we want to share that ownership with variables so we can access each `Node` in the tree directly, to do this, we define a Vec<T> of Rc<Node>
    // We also want to modify which nodes are children of another node, so we have a RefCell<T> in children around the Vec<Rc<Node>>
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // A `leaf`` Node instance with no children
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    // A `branch` Node instance with `leaf` as one of its children
    let branch = Rc::new(Node {
        value: 5,
        // Clone the Rc<Node> in `leaf`
        // The Node in `leaf` now has two owners: `leaf` and `branch`
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
*/

// We can get from `branch` to `leaf` through `branch.children`, but there is no way to get from `leaf` to `branch`
// The reason is that `leaf` has no reference to `branch`

//
// Adding a Reference from a Child to Its Parent
//

// We need to add a `parent` field to our Node struct definition
// It can't be an Rc<T>, because that would create a reference cycle between `leaf.parent` pointing to `branch` and `branch.children` pointing to `leaf`

// Thinking of relationships in another way: when a parent node is dropped, its child nodes should be dropped too
// However, a child should not drop its own parent
// This is a case for weak references

// Make the `parent` use Weak<T>, specifically a RefCell<Weak<Node>>

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// A node will be able to refer to its parent node, but doesn't own its parent

/*
fn main() {
    // `leaf` starts out with no parent
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // We will get a `None` value here
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Modify `leaf` to give it a Weak<Node> reference to its parent
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // We will get a `Some` variant holding `branch`
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
*/

//
// Visualizing Changes to `strong_count` and `weak_count`
//

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // leaf strong = 1, weak = 0
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // branch strong = 1, weak = 1
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        // leaf strong = 2, weak = 0
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    // leaf parent = None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // leaf strong = 1, weak = 0
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
