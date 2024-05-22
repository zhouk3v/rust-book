// `Drop` is a trait that customizes what happens to a value when it is about to go out of scope
// This is useful to release resources such as files and network connections

// The functionality of the `Drop` trait is almost always used when implementing a smart pointer
// Eg: If a Box<T> is dropped, it will deallocate the space on the heap that it points too

// With the Drop trait, you can specify code to run whenever a value goes out of scope, and the compiler will insert this code automatically
// This reduces the need for "cleanup" code to drop resources

// The Drop trait requires the implementation of one method: drop() that takes a mutable reference to `self`

// A CustomSmartPointer struct that implements the Drop trait
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // The body of the drop method is where you would place logic that runs when an instance of the type goes out of scope
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

/*
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
*/

// Note that variables are dropped in the reverse order of creation

//
// Dropping a Value Early with `std::mem::drop`
//

// Some a value might be needed to be cleaned up early
// Eg. Using the drop() method to release a lock

// Rust will not allow you to call the Drop trait's drop() method manually
/*
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointer created");
    // Won't work
    c.drop();
    println!("CustomSmartPointer dropped before the end of main");
}
*/

// The error message that appears when calling the drop() method manually mentions the term `destructor`, which drop() is an example of
// Rust does not allow us to call the drop() method manually since it still automatically call drop() on the value at the end of the scope
// This will result in a double free error
// Note that the automatic call to drop() cannot be disabled

// To drop a value before the end of scope, use the `std::mem::drop` function, which takes one argument - the value we want to drop
/*
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointer created");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}
*/

// ASIDE - not in book

// Think of std::mem::drop() as a generic function with an empty body
fn customDrop<T>(x: T) {}
// The drop() function takes ownership of the value, and when the function goes out of scope, the value that the function takes ownership of is dropped
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointer created");
    customDrop(c);
    println!("CustomSmartPointer dropped before the end of main");
}
// END ASIDE

// Note that the ownership rules prevent the usage of values dropped early - the ownership system makes sure that references are always valid,
// and also checks that drop() is called only once when the value is no longer used
