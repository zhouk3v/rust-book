// Unsafe Rust doesn't enforce memory safety guarantees

// It exists because static analysis is conservative: it is better for the compiler to reject some valid programs than to accept some invalid programs
// If the compiler doesn't have enough information to be confident, it will reject the code
// In this case, unsafe code can be used to tell the compiler that the code was manually verified to be safe
// However, using unsafe code incorrectly can cause problems due to memory unsafety, such as null pointer dereferencing

// Another reason is that the underlying computer hardware is inherently unsafe
// Rust needs to allow for low-level systems programming, such as directly interacting with the operating system or writing an operating system

//
// Unsafe Superpowers
//

// To switch to unsafe Rust, use the `unsafe` keyword and then start a new block that holds the unsafe code
// There are five actions exclusive to unsafe Rust
// - Dereference a raw pointer
// - Call an unsafe function or method
// - Access or modify a mutable static variable
// - Implement an unsafe trait
// - Access fields of a `union`

// Note that `unsafe` doesn't turn off the borrow checker or disable any other of Rust's safety checks

// In addition, `unsafe` does not mean the code inside the block is necessarily dangerous or that it will definitely have memory safety problems
// The intent is that the programmer will enusre that code in an `unsafe` block will access memory in a valid way

// As a result of requiring the five unsafe operations to be inside blocks annotated with `unsafe`, any memory safety errors will be limited to those blocks
// Keep `unsafe` blocks small, to help debug memory bugs

// To isolate unsafe code as much as possible, it's best to enclose unsafe code within a safe abstraction and provide a safe API
// Parts of the standard library are implemented as safe abstractions over unsafe code that has been audited
// Wrapping unsafe code in a safe abstraction prevents uses of `unsafe` from leaking out into all the places that you or other users might want to use the functionality implemented with unsafe code
// because using a safe abstraction is safe

/*
use std::slice;

fn main() {
    //
    // Dereferencing a Raw Pointer
    //

    // Unsafe Rust has two new types called raw pointers that are similar to references
    // Raw pointers can be immutable or mutable and are written as `*const T` and `*mut T` respectively
    // (The asterisk is part of the type name)
    // In the context of raw pointers, immutable means that the pointer can't be directly assigned to after being dereferenced

    // Compared to references and smart pointers, raw pointers:
    // - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers, or multiple mutable pointers to the same location
    // - Aren't guaranteed to point to valid memory
    // - Are allowed to be null
    // - Don't implement any automatic cleanup

    // By opting out of these guarantees, you give up guaranteed safety in exchange for greater performance or the ability to interface with another language,
    // or hardware where Rust guarantees don't apply

    // Creating an immutable and a mutable raw pointer from references
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Note the lack of the `unsafe` keyword, we can create raw pointer in safe code, we just can't dereference raw pointers outside an unsafe block

    // We create raw pointers by using `as` to case an immutable and a mutable reference into their corresponding raw pointer types
    // Because we created them directly from references guaranteed to be valid, we know that these raw pointers are valid
    // but this assumption doesn't apply to every raw pointer

    // Creating a raw pointer whose validity is uncertain by creating a pointer to an arbitary location in memory
    // Trying to use arbitary memory is undefined
    // - There might be data at that address or there might be not
    // - The compiler might optimize the code so there is no memory access
    // - The program might error with a segmentation fault
    let address = 0x012345usize;
    let r = address as *const i32;
    // Not in book: printing out the memory address stored in `address` (which should be the same as what we specified above)
    println!("{:p}", r);

    // Using the dereference operator `*` on a raw pointer requires the `unsafe` block
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Creating a raw pointer is harmless, it is only when we try to access the value that it points at that we might end up dealing with an invalid value

    // Note that we created an immutable `*const i32` and a mutable `*mut i32` raw pointer that both pointed to the same memory location where `num` is stored
    // If we tried to create an immutable reference and a mutable reference to `num` instead,
    // a compiler error will be thrown (an immutable reference cannot exist at the same time as a mutable reference)
    // With raw pointers, we can create an imutable pointer and a mutable pointer at the same location and change data through the mutable pointer,
    // which can create a data race

    // One major use case for raw pointers is interfacing with C code
    // Another case is when building up safe abstractions that the borrow checker doesn't understand

    //
    // Calling an Unsafe Function or Method
    //

    // Unsafe functions and methods look like regular functions and methods, but they have an extra `unsafe` before the rest of the definition
    // The `unsafe` keyword in this context indicates the function has requirements that we need to uphold when we call this function,
    // because Rust can't guarantee we've met these requirements.
    // By calling an unsafe function within an `unsafe` block, we're saying that we've read the function's documentation,
    // and take responsibility for upholding the function's contracts

    unsafe fn dangerous() {}

    unsafe {
        // We must call dangerous() within a seperate `unsafe` block
        // A compiler error will be thrown if we try to call it outside an `unsafe` block
        dangerous();
    }

    // Bodies of unsafe functions are effectively `unsafe` blocks, so we can perform other unsafe operations within an unsafe function without adding another `unsafe` block within it

    //
    // Creating a Safe Abstraction over Unsafe code
    //

    // A function containing unsafe code doesn't mean we need to mark the entire function as unsafe
    // Example: split_at_mut() from the standard library
    // - This method takes one mutable slice and makes two mutable slices by splitting the slice at the index given as an argument

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Note that this function is impossible to implement in safe Rust
    // Example: this attempt doesn't work
    /*
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();

        assert!(mid <= len);
        // Error thrown here - there are two mutable references to one value
        // Even though we are mutably borrowing different parts of the slice, Rust doesn't know that, it only sees us borrowing the same slice twice
        (&mut values[..mid], &mut values[mid..])
    }
    */

    // When we know that the code is ok, but Rust doesn't, use the `unsafe` block
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        // Use len() to get the length of the slice
        let len = values.len();
        // Use the as_mut_ptr() method to access the raw pointer of a slice (the memory address of the begining of the slice)
        // Because we have a mutable slice to i32 values, as_mut_ptr() returns a raw pointer with the type *mut i32 that is stored in the `ptr` variable
        let ptr = values.as_mut_ptr();

        // Assert that `mid` is within the slice
        assert!(mid <= len);

        unsafe {
            (
                // slice::from_raw_parts_mut() takes a raw pointer and a length, and it creates a slice
                // Using it, we create a slice that starts from `ptr` and is `mid` items long
                slice::from_raw_parts_mut(ptr, mid),
                // Use the add() method on the `ptr` raw pointer to get a raw pointer that starts at `mid`
                // And then create a slice using that pointer that the remaining number of items after `mid` as the length
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    // The function slice::from_raw_parts_mut() is unsafe because it takes a raw pointer and must trust that this pointer is valid
    // The add() method on raw pointers is also unsafe, because it must trust that the offset location is also a valid pointer
    // Thus, these methods must be within an `unsafe` block
    // By looking at the code and adding the assertion that `mid` must be less than or equal to `len`,
    // we can tell that all the raw pointers used in the `unsafe` block will be valid pointers to data within the slice

    // Note that we don't need to mark the resulting split_at_mut() function as unsafe, and thus we can call this function from safe Rust
    // This is an example of a safe abstraction to the unsafe code with an implementation of the function that uses `unsafe` code in a safe way,
    // because it creates only valid pointers from the data this function has access to.

    // In contrast, this code, which takes an arbitary memory location and create a slice 10000 items long would likely crash
    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    // We don't own the memory at this arbitary location, and there is no guarantee that the slice this code creates contains valid `i32` values
    // Attempting to use `values` as though it's a valid slice results in undefined behaviour
    /*
    // Not in book
    println!("{}", values[0]);
    */

    //
    // Using `extern` Functions to Call External Code
    //

    // Rust code might need to interact with code written in another language
    // For this, Rust has the `extern` keyword that facilitates the creation and use of a Foreign Function Interface (FFI)
    // An FFI is a way for a programming languae to define functions and enable a different (foreign) programming language to call those functions

    // Setting up an integration with the abs() function from the C standard library

    // Within the `extern "C"` block, we list the names and signatures of external functions from another language we want to call.
    // The "C" part defines which application binary interface (ABI) the external function uses
    // The ABI defines how to call the function at the assembly level
    // The "C" ABI is the most common and follows the C programming language's ABI
    extern "C" {

        fn abs(input: i32) -> i32;
    }

    // Functions declared within `extern` blocks are always unsafe to call from Rust code
    // This is because other languages don't enforce Rust's rules and guarantees, and Rust can't check them
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    //
    // Calling Rust Functions from other languages
    //

    // `extern` can also be used to create an interface that allows other languages to call Rust functions
    // We add the `extern` keyword and specify the ABI to use just before the `fn` keyword
    // We also need to add the `#[no_mangle]` annotation to tell the Rust compiler not to mangle the name of the function

    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    // This usage of `extern` does not require `unsafe`
}
*/

//
// Accessing or Modifying a Mutable Static Variable
//

// In Rust, global variables are called static variables
/*
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
*/

// Static variables are similar to constants
// The names of the static variables are in SCREAMING_SNAKE_CASE by convention
// Static variables can only store references with the `'static` lifetime,
// which means the Rust compiler can figure out the lifetime and we aren't required to annotate it explicitly
// Accessing an immutable static variable is safe

// One difference between constants and immutable static variables is that values in a static variable have a fixed address in memory
// Using the value will always access the same data.
// Constants are allowed to duplicate their data whenever they are used

// Another difference is that static variables can be mutable
// Accessing and modifying mutable static variables is unsafe

// We specify mutability using the `mut` keyword
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    // Any code that reads or writes from `COUNTER` must be within an `unsafe` block
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    // Reading from `COUNTER` (again, must be within an `unsafe` block)
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// This is because with mutable data that is globally accessible, it is difficult to ensure that there are no data races
// Where possible, it is preferred to use the concurrency techniques and thread-safe smart pointers instead

//
// Implementing an Unsafe Trait
//

// `unsafe` can be used to implement an unsafe trait
// A trait is unsafe when at least one of its methods has some invariant that the compiler can't verify

// Declare a trait is `unsafe` by adding the `unsafe` keyword before `trait`
unsafe trait Foo {}

// Mark the implementation of the trait as unsafe too
// By using `unsafe impl`, we're promising that we'll uphold the invariants that the compiler can't verify
unsafe impl foo for i32 {}

// Example: `Sync` and `Send` marker traits
// If we implement a type that contains a type that is not `Send`, such as raw pointers,
// and we want to mark that type as `Send` or `Sync`, we must use `unsafe`
// Rust can't verify that our type upholds the guarantees that it can be safely sent across threads or accessed from multiple threads
// Thus, we need to do those checks manually and indicate as such with `unsafe`

//
// Accessing Fields of a Union
//

// A `union` is similar to a `struct`, but only one declared field is used in a particular instance at one time.
// Unions are primarily used to interface with unions in C code
// Accessing union fields is unsafe because Rust can't guarantee the type of the data currently being stored in the union instance

//
// When to use Unsafe code
//

// Using `unsafe` isn't wrong or frowned upon
// But it is tricker to get `unsafe` code correct because the compiler can't help uphold memory safety
