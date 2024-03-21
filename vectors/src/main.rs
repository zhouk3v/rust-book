fn main() {
    //
    // Creating a new vector
    //

    /*
    // Call the Vec::new() function to create a new vector
    // We'll need a type annotation for an empty vector
    // Vectors can hold any type, due to generics (the <> brackets), in this case, the vector holds i32 types
    // Remember that Vectors are heap-allocated
    let v: Vec<i32> = Vec::new();

    // The vec! macro create a new vector with initial values
    // From there, rust can infer what Data type the vector holds
    let v = vec![1, 2, 3];
     */

    //
    // Updating a vector
    //

    /*
    // To add elements to a vector, use the push method
    // Make sure to make it mutable!
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
     */

    //
    // Reading elements of vectors
    //

    /*
    let v = vec![1, 2, 3, 4, 5];

    // First method: indexing with []
    // Vectors are zero-indexed

    // Using & and [] gives us a reference to the element at the index value
    // It is best practice to use a reference in conjuction with indexing, as the objects that the vector stores can either be stack or heap allocated
    let third: &i32 = &v[2];

    // Works in this case, as i32 is a value that is simply copied over
    let first = v[0];

    // Heap allocated objects cannot be moved out (unless they implement Copy)
    let v_str = vec![String::from("Hello"), String::from("world")];
    // Doesn't work
    //let first = v_str[0];
    // Alternative, use a reference
    let first = &v_str[0];

    println!("The third element is {third}");

    // Second method: get()
    // get returns an Option<&T> that can be used with match
    // Note that the element inside the option is a reference!
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Each method responds differently if the index is out of bounds
    // [] causes the program to panic if the index is out of bounds
    //let does_not_exist = &v[100];
    // get() will return None if the index is out of bounds
    let does_not_exist = v.get(100);

    // The borrow checker will enforce the ownership and borrowing rules to ensure that any references to the vector created by & [] and get() will remain valid
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // Won't work, as the immutable reference under 'first' is still live (push() implicitly uses a mutable reference)
    // push() might allocate new memory and deallocate the old memory to fit the new element, which would mean that the reference under 'first' would point to deallocated memory (undefined behaviour)
    v.push(6);

    println!("The first element is: {first}");
    */

    //
    // Iterating over the Values in a Vector
    //

    /*
    let v = vec![100, 32, 57];

    // Using a for loop to iterate through a vector
    for n_ref in &v {
        // Remember to use the dereference operator to read the i32
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }

    // We can also iterate over mutable reference to each element in a mutable vector
    // Note that since there is a reference to an element in the vector, the vector object itself will temporarly lose permissions (all permissions in this case)
    let mut v = vec![100, 32, 57];
    for n_ref in &mut v { //NOTE: the &mut is referencing the element in the vector, NOT the vector itself
        *n_ref += 50;
    }
     */

    //
    // Safely Using Iterators
    //

    /*
    let mut v: Vec<i32> = vec![1, 2];
    // The iterator is a pointer (reference) that moves through each element of the vector.
    let mut iter = v.iter();
    // The 'next()' method advances the iterator and returns an optional reference (Option<&T>) to the previous element, which will be either Some or None at the end of the vector
    let n1 = iter.next().unwrap();
    let n2 = iter.next().unwrap();
    // iter will return None here as the iterator has reached the end of the array
    let end = iter.next();

    // Iterators implicitly use an inmutable reference, so we can't modify the vector when the iterator is active
    // (The write permission is removed on the vector (*v))
    /*
    fn dup_in_place(v: &mut Vec<i32>) {
        for n_ref in v.iter() {
            // This won't work, as v is immutably borrowed by the iterator, but push() needs to borrow it mutably
            // In terms of permissions, push() needs the write permission on *v, but that permission on *v is temporarily removed when the iterator is active
            // The safety issue here is that push() might deallocate the memory that the vector is currently in, which would leave the iterator's pointer to point at freed memory
            v.push(*n_ref);
        }
    }
    */

    // You can also iterate over a vector with a range, which does not use a pointer. The iterator will return vector indexes
    let mut v = vec![1, 2];
    let mut iter = 0..v.len();
    let i1 = iter.next().unwrap();
    let n1 = &v[i1];
     */

    //
    // Using an Enum to Store Multiple types
    //

    /*
    // Vectors can only store one type, but we can get around that by storing an enum in the vector instead

    // Define an enum who's variants define the different types to hold and store the actual values
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Create a vector which holds instances of the enum
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Note that we need know the exhaustive set of types that a vector will need to store, or else the enum method won't work (use a trait instead)
     */

    //
    // Dropping a Vector Drops its Elements
    //

    // A vector and all of its contents (stack or heap allocated) are dropped when it goes out of scope
    {
        let v = vec![1, 2, 3, 4];
    } // v goes out of scope and is dropped here
}
