fn main() {
    //
    // Variable scope
    //

    /* 
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
    */
    
    //
    // The String type
    //

    // A complex type that is stored on the heap, as compared to string literals which are stored on the stack

    /*
    let s = String::from("hello");
    */

    // The String type can be mutated
    /*
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    */

    //
    // Memory and allocation
    //

    // String and other complex data types (i.e. anything that is not a primitive literal such as i64, u32, f64, etc) need to be allocated on the heap
    // Rust will take care of allocation and deallocation through ownership


    /*
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
    */

    // The memory used for s is automatically freed when s is no longer valid
    // Rust will automatically call a special function called 'drop' to free the memory

    //
    // Variables and Data Interacting with move
    //

    // Scalar data types which are stored on the stack are simply copied over
    /*
    let x = 5;
    let y = x;
    println!("{} {}", x, y)
    */
    // x will be assigned a value of 5, when y is created, it will copy the value 5 from x. Both variables are still valid

    // Complex (heap-allocated) data types will store information such as the pointer (location of the object's data in the heap), length (how much memory is currently being used by the object) and capacity (amount of memory received from the allocator for the object) onto the stack
    /*
    let s1 = String::from("hello");
    let s2 = s1;
    // When we assign s2 to s1, we copy over the pointer, length and capacity, but NOT the data in the heap
    // To avoid a double-free, Rust will consider s1 to be invalid after copying the stack data to s2.
    // Invalidating s1 will solve the problem of trying to deallocate both s1 and s2 when they go out of scope, but both s1 and s2 point to the same location, which would result in a double free
    // The act of invalidating s1 here is considered a 'move'
    
    // This won't work, s1 is invalid
    // println!("{}, world!", s1);
    */

    //
    // Variables and Data Interacting with Clone
    //

    // Most heap-allocated data types will provide a clone function to deep copy (i.e. copy both the stack and heap data)
    // Note that clone() is an expensive operation
    /*
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{} {}", s1, s2);
    */

    //
    // Stack only data: Copy
    //

    // Stack allocated types are trivially copied: y will copy the value '5' from x
    /*
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    // The reason why values can be copied is that integers (and other stack allocated types) have a known size at compile time
    // Rust has a 'copy' trait for types (stack-allocated types), where if it's implemented, will trivially copy the data rather than move it, letting both variables assigned to the data still valid
    // 'Copy' is implemented in all scalar types in Rust (u64, i32, bool)
    // 'Copy' cannot be implemented on types with the 'drop' trait. The 'Drop' trait means that something more has to be done when the instance of the type goes out of scope (i.e. heap memory needs to be de-allocated)
    */

    //
    // Ownership and Functions
    //

    /*
    // Passing a variable will move or copy, depending on the type
    {
        let s = String::from("hello");  // s comes into scope
    
        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here
    
        let x = 5;                      // x comes into scope
    
        makes_copy(x);                  // x would move into the function,
                                        // but i32 is Copy, so it's okay to still
                                        // use x afterward
        // This won't work, as s is invalid and the original value was dropped after takes_ownership was called
        // println!("{s}");
        // This will work, as x is still valid
        println!("{x}");
    
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.
    
    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
      // memory is freed.
    
    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.
    */

    //
    // Return Values and Scope
    //

    // Function return values transfer ownership to the variable in which the function call is assigned to
    /*
    {
        let s1 = gives_ownership();         // gives_ownership moves its return
                                            // value into s1
    
        let s2 = String::from("hello");     // s2 comes into scope
    
        let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3
        // Works, return value of gives_ownership was moved to s1
        println!("{s1}");
        // Doesn't work, the value was moved when takes_and_gives_back was called and thus s2 is now invalid
        // println!("{s2}");
        // Works, return value of takes_and_gives_back was moved to s3
        println!("{s3}");
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.
    
    fn gives_ownership() -> String {             // gives_ownership will move its
                                                 // return value into the function
                                                 // that calls it
    
        let some_string = String::from("yours"); // some_string comes into scope
    
        some_string                              // some_string is returned and
                                                 // moves out to the calling
                                                 // function
    }
    
    // This function takes a String and returns one
    fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                          // scope
    
        a_string  // a_string is returned and moves out to the calling function
    }
    */

    // Rust allows us to return multiple values from functions via tuples
    {
        let s1 = String::from("hello");
    
        // s1 is invalidated here
        let (s2, len) = calculate_length(s1);
    
        println!("The length of '{}' is {}.", s2, len);
    }
    
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
    
        (s, length)
    }
}
