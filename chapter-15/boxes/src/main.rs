// Boxes allow storing data on the heap rather than on the stack

// They are often used in these situations:
// - When a type whose size can't be known at compile time, and you want to use a value of that type in a context that requires an exact size
// - When a large amount of data needs to be moved (transferred ownership) without copying
// - When you want to own a value and care only that it's a type that implements a particular trait rather than being of a specific type

//
// Using a Box<T> to Store Data on the Heap
//

/*
fn main() {
    // Store an i32 value on the heap
    let b = Box::new(5);
    // Print out `b = 5`
    println!("b = {}", b);
    // The box will be deallocated when out of scope
}
 */

//
// Enabling Recursive Types with Boxes
//

// A value of a recursive type can have another value of the same type as part of itself (i.e. Linked List)
// They pose a problem in Rust, since it needs to know how much space a type takes up at compile time
// However, the nesting of values in recursive types could be infinite
// Since boxes have a known size, we can use a box containing the value in the recursive type definition

// An example is the cons list - a linked list equivelent in functional programing languages

//
// More Information About the Cons List
//

// A cons list is made up of nested pairs, which come from a `cons` function, that constructs a new pair from its two arguments
// By calling `cons` on a pair with a value and another pair, we can construct a cons list made up of recursive pairs (pair within pair)

// Eg: (1, (2, (3, Nil)))

// A non-working implementation of a cons list via enums
// The problem here is that the type has infinite size, since `List` with a variant that is recursive - it holds another value of itself directly
// Rust can't figure out how much space it needs to store a `List` value here
/*
enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
 */

//
// Computing the size of a non-recursive type
//

// For an enum, Rust will go through each of the variants to see which variant needs the most space
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// For the `Message` enum, Rust sees that Message::Quit doesn't need any space, Message::Move needs space for two i32 values, and so on
// Since only one variant will be used, the most space a `Message` enum value will need would be equivlent to the space needed to store the largest variant

// For the `List`` enum, Rust will see that the `Cons`` variant will hold an i32 value and a `List` value.
// Thus, `Cons` will need the space of an i32 value + a `List` value.
// Rust will then figure out how much space a `List` enum will take again, and would continue infinitely

//
// Using Box<T> to get a recursive type with a known size
//

// The compiler will suggest some "indirection" to fix the problem with the first implementation of the Cons list
// "Indirection" means to store a value indirectly by storing a pointer to the value instead

// Box<T> is a pointer, so Rust will always know how much space a Box<T> needs (a pointer's size does not change based on the amount of data it is pointing to)

// We can put a Box<T> inside the Cons variant of the List enum
// The Box<T> will point to the next List value that will be on the heap rather than inside the Cons variant
// Think of this implementation as putting the items next to each other rather than inside each other (like a linked list!)

enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Not in book - example of a print method for a List
impl List {
    fn print(&self) {
        match self {
            List::Cons(x, next) => {
                println!("{x}");
                next.print();
            }
            List::Nil => (),
        }
    }
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    list.print();
}

// Here, the Cons variant needs the size of an i32 plus the space to store the box's pointer data.
// Thus, any List value will take up the size of an i32 plus the size of a box's pointer data.

// Boxes only provide indirection and heap allocation, they don't have any other special capabilities.
// They also don't have performance overhead that these special capabilities incur
// Thus, they are useful where indirection is the only feature needed

// The Box<T> is a smart pointer because it implements the `Deref` trait, which allows Box<T> values to be treated like references
// When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up due to the `Drop` trait implementation
