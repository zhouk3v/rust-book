fn main() {
    //
    // References and borrowing
    //

    /*
    // References are address that point to an object in the heap. It is guarenteed that they will point to a valid object
    {
        let s1 = String::from("hello");
        
        // Use a '&' before an object name to create a reference
        // The &s1 reference will refer the value of s1, but it does not own it
        // The action of creating a reference is called borrowing
        let len = calculate_length(&s1);

        // s1 is still valid here, as we just pass in a reference to calculate_length instead of moving it
        println!("The length of {s1} is {len}.")
    }
    // References are annotated with a '&' next to the type in the function signature
    fn calculate_length(s: &String) -> usize { // s is a reference to a String
        s.len()
    } // Here, s goes out of scope. But because it does not have ownership of what
      // it refers to, the value that it represents is not dropped.
    */

    /*
    // References by default are immutable (cannot modify the data that it is borrowing), so this function won't work
    {
        let s = String::from("hello");
    
        change(&s);
    }
    
    // fn change(some_string: &String) {
    //     some_string.push_str(", world");
    // }
    */

    //
    // Mutable references
    //

    /*
    // Use a mutable reference if you need to modify the data behind a reference
    {
        // change variable to be mutable
        let mut s = String::from("hello");
    
        // Add in 'mut' after the & to pass in a mutable reference
        change(&mut s);
        println!("{s}");
    }
    
    // Annotate the reference in the function signature with 'mut' to make it clear that the function will modify the data that it borrows
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    */

    // If a mutable reference is created for a value, then no other references can be created when that reference exists(askin to a reader-writer lock)
    // This rule is in place to prevent data races
    /*
    let mut s = String::from("hello");

    let r1 = &mut s;
    // This will fail, as there is already a mutable reference for s (r1)
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
    */
    /*
    // This works, as the first mutable reference is dropped (goes out of scope) before the second muttable reference is created
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    */
    
    // Immutable references also can't be created on a value if a mutable reference exists on it (again, like a rw lock)
    /*
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
    */

    // Note that a reference's scope begins when it is created, and ends when it is last used,
    /*
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point (their scope ends here)

    let r3 = &mut s; // no problem, as r1 and r2 are invalid now
    println!("{}", r3);
    */

    //
    // Dangling references
    //

    // Rust will check for dangling references (references that might point to data that is now allocated to someone else) at compile time
    /* 
    fn main() {
        let reference_to_nothing = dangle();
    }
    
    // Won't compile, as the reference dangle() will return is a dangling reference
    // fn dangle() -> &String { // dangle returns a reference to a String

    //     let s = String::from("hello"); // s is a new String
    
    //     &s // we return a reference to the String, s
    // } // Here, s goes out of scope, and is dropped. Its memory goes away.
    //   // Danger!

    // Proper implementation, function will transfer ownership of s to caller
    fn no_dangle() -> String {
        let s = String::from("hello");
    
        s
    }
    */
}
