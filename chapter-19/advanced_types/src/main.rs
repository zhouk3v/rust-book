//
// Using the Newtype Pattern for Type Safety and Abstraction
//

// The newtype pattern is also useful for statically enforcing that values are never confused and indicating the units of a value
// Eg: `Millimeters` and `Meters` structs wrapping `u32` values in a newtype
// We can make sure that a function that expects a `Millimeters` value to not be called with a `Meters` value or a regular `u32`

// The newtype pattern can also abstract away some implementation details of a type
// In other words, the newtype pattern can expose a public API that is different from the API of the private inner type.

// Newtypes can also hide internal implementation
// Eg. a `People` type that wraps a `HashMap<i32, String>` to store a person's ID associated with their name
// For the public API, we can write a method to add a name string to the `People` collection
// That code wouldn't need to know that we assign an `i32` ID to names internally
// The newtype pattern is another example of achieving encapsulation to hide implementation details

fn main() {
    //
    // Creating Type Synonyms with Type Aliases
    //

    // Rust has the ability to declare a type alias to give an existing type another name with the `type` keyword
    type Kilometers = i32;
    // The alias `Kilometers` is a synonym for `i32` (it is not a seperate, new type)
    // Values that have the the type `kilometers` will be treated the same as values of type `i32`
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // Because `Kilometers` and `i32` are the same type, we can add values of both types and we can pass `Kilometers` values to functions that take `i32` parameters
    // However, we don't get the type checking benefits that we get from the newtype pattern

    // The main use case is to reduce repetition with lengthy types
    // E.g. a `Thunk` type for the `Box<dyn Fn() + Send + 'static>`
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi!"));

    fn takes_long_type(f: Thunk) {}

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("hi!"))
    }

    // Type aliases are also commonly used with `Result<T,E>`
    // E.g. the `std::io` module: I/O operations often return a `Result<T,E>` to handle situations when operations fail
    // The library has a `std::io::Error` struct that represents all possible I/O errors
    // Many of the functions in `std::io` will return a `Result<T,E>` where the `E` is `std::io::Error`

    /*
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
        fn flush(&mut self) -> Result<(), Error>;

        fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    }
    */

    // Since `Result<..., Error>` is repeated a lot, `std::io` has the following type alias declaration:
    /*
    type Result<T> = std::result::Result<T, std::io::Error>;
     */

    // We can use the fully qualified alias `std::io::Result<T>`, which is a `Result<T,E>` with the `E` filled in as `std::io::Error`
    /*
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }
    */

    // Note that since `std::io::Result<T>` is an alias, we can use any methods on `Result<T,E>` with it, as well as syntax like the `?` operator

    //
    // The Never Type that Never Returns
    //

    // The `!` type is known as the "never" type as it stands in the place of the return type when a function will never return

    /*
    fn bar() -> ! {}
    */
    // The above code is read as "the function bar() returns never"
    // Functions that return never are called "diverging functions"
    // We can't create values of type `!` so bar() can never possibly return

    // This is used in `match` when the arm returns `continue`
    /*
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    */
    // Remember that all match arms must return the same type
    // A `continue` expression has the type `!`
    // In other words, when Rust computes the type of `guess`,
    // it looks at both match arms, the first arm with a value of u32, and the latter with a `!` value
    // Because `!` can never have a value, Rust decides that the type of `guess` is `u32`

    // Expressions of type `!` can be coerced into any other type
    // A `match` arm can end in `continue` because `continue` doesn't return a value,
    // instead it moves control back to the top of the loop,
    // so in the `Err` case, we never assign a value to `guess`

    // The `!` type is useful with the `panic!` macro as well

    // E.g. the unwrap() definition for `Option<T>`
    /*
    impl<T> Option<T> {
        pub fn unwrap_def(self) -> T {
            match self {
                Some(val) => val,
                None => panic!("called `Option::unwrap()` on a `None` value"),
            }
        }
    }
    */

    // `panic!` has the type `!`, so the result of the `match` expression is `T`
    // `panic!` doesn't produce a value, it ends the program so in the `None` case, we won't be returning a value from unwrap()

    // The `loop` also has a `!` type
    print!("forever ");
    loop {
        println!("and ever ")
    }
    // This loop never ends, so `!` is the type of the expression
    // This wouldn't be true if the loop has a `break`, because the loop would terminate when `break` is reached

    //
    // Dynamically Sized Types and the `Sized` Trait
    //

    // Usually, Rust needs to know how much space to allocate for a value of a particular type
    // However, there exists "dynamically sized types", types that let us write code using values whose size that can only be known at runtime

    // `str` is an example of a dynamically sized type, (not to be confused with `&str`)
    // We can't know how long the string is until runtime, meaning that we can't create a variable of type `str`, or take an argument of type `str`

    // This doesn't work
    /*
    let s1: str = "Hello there!";
    let s2: str = "How's it going?"
    */

    // Rust needs to know how much memoru to allocate for any value of a particular type,
    // and all values of a type must use the same amount of memory
    // The two `str` value above would need to take up the same amount of space, but they have different lengths:
    // `s1` needs 12 bytes of storage, and `s2` needs 15 bytes
    // This is why it's not possible to create a varibale holding a dynamically sized type

    // To fix this, make `s1` and `s2` a `&str` (what we've been using throughout the book)
    // Remember that a slice data structure `&str` stores the starting position and the length of the slice
    // In contrast to a reference `&T` which is a single value that stores the memory address of where `T` is located,
    // `&str` is two values: the address of the `str` and its length
    // This, we can know the size of a `&str` at compile time: it's twice the length of `usize`, regardless the length of the string

    // This is the general way of how dynamically sized types are used in Rust: they has some extra metadata that stores the size of the dynamic information
    // The rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind
    // Eg: we can combine `str` with pointers such as `Box<str>` or `Rc<str>`

    // Traits are also a dynamically sized type that we can refer to by using the name of the trait
    // When using traits as trait objects, we must put them behind a pointer, such as `&dyn Trait` or `Box<dyn Trait>`

    // To work with DSTs, Rust provides the `Sized` trait to determine whether or not a type's size is known at compile time
    // This trait is automatically implemented for everything whose size is known at compile time
    // In addition, Rust implicitly adds a bound on `Sized` to every generic function:

    // This definition:
    fn generic<T>(t: T) {}

    // is actually treated as though we had written this:
    fn generic<T: Sized>(t: T) {}

    // Generic functions will work only types that have a known size at compile time by default
    // However, you can use the `?Sized` syntax to relax this restriction
    fn generic<T: ?Sized>(t: &T) {}

    // A trait bound on `?Sized` means "`T` may or may not be `Sized`" and this notation overrides the default that generic types must have a known size at compile time
    // The `?Trait` syntax with this meaning is only available for the `Sized` trait

    // Note that the `t` parameter is now a `&T`. Because the type might not be `Sized`, we need to use it behind some kind of pointer
}
