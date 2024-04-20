//
// In Function Definitions
//

/*
// To parameterize types in a function, we need to name the type parameter
// Rust convention is to use UpperCamelCase
// The default type name is 'T'

// We declare type parameters with <> brackets in the function signature (The function is generic over type T)
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        // Note that the type T must implement the trait std::cmp::PartialOrd in order to use comparision operators,
        // so this function won't compile for now, as we haven't stipulated that
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    // Call the function as normal (i.e., you don't need to stipulate the type being used when calling the function)
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    // The function should work with a vec of a different type
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
*/

//
// In struct Definitions
//

/*
// A Point struct to hold a x and y coordinate values of any type
// First, declare a type name in <> brackets
struct Point<T> {
    // Then use the type in the struct definition (i.e. defining fields)
    x: T,
    y: T,
}

fn main() {
    // Use the generic struct as usual (i.e. you don't need to stipulate the type)
    let integer = Point { x: 10, y: 5 };
    let float = Point { x: 2.0, y: 3.0 };
    // Note that since x and y both are the generic type T in the struct definition, x and y need to be both the same type
    // The below will fail to compile
    //let wont_work = Point { x: 5, y: 4.0 };
}
*/

/*
// We can use multiple generic types (in order to fix the above problem of x and y being different types)
// We can have as many generic types as we want, but too many of them can make the code hard to read
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    // Now, x and y can be the same or different types!
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 2.0, y: 5.0 };
    let integer_and_float = Point { x: 5, y: 8.0 };
}
*/

//
// In-Enum Definitions
//

/*
// We can define enums which hold generic data types in their variants

// A built-in example in Rust is the Option type
enum OptionDef<T> {
    Some(T),
    None,
}

// Enums can also use multiple generic types

// The Result type in Rust is an example of an enum that uses two generic types
enum ResultDef<T, E> {
    Ok(T),
    Err(E),
}

*/

//
// In Method Definitions
//

// We can use generic types with methods on structs and enums.

/*
struct Point<T> {
    x: T,
    y: T,
}
// Declare the generic type (T) just after `impl`
// This will tell Rust that the type T in Point<T> right after the impl<T>, is a generic type rather than a concrete type
// We can use any name for this generic type for `impl`, but using the same name as the generic type in the struct/enum is conventional
// - NOTE: If you want to use a different name, write `impl<F> Point<F>` and use the `F` type in the methods instead
// Methods written in this `impl` block that declares a generic type will be defined on any instance of the type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// We can specify constraints on generic types when defining methods on the type
// i.e. implement methods only for Point instances that contain floating point numbers
// We don't declare a generic type for impl in this case
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        // The powi() and sqrt() functions are only available for floating point numbers
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 50, y: 10 };
    println!("p.x = {}", p.x());
}
*/

// Generic type parameters in a struct definition aren't always the same as the ones used in the same struct's method signatures
// This allows us to work with parameters which are instances of a struct with generic types whose types do not match the caller

// This Point sturct definitions uses generic types X1 and Y1
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// The generic parameters X1 and Y1 are declared with the impl block as they go with the struct definition
impl<X1, Y1> Point<X1, Y1> {
    // But the mixup method uses the X2 and Y2 generic types instead.
    // The generic parameters X2 and Y2 are declared here as they are only relavent with the function
    // This allows the Point instance passed into `other` to have different types than the Point calling mixup
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }

    // NOTE: Not in book
    // The mixupSameType method doesn't have generic parameters declared
    // As a result, the passed in point in `other` needs to have matching types with the instance of the point calling mixupSameType for every property
    fn mixupSameType(self, other: Point<X1, Y1>) -> Point<X1, Y1> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // p1 has an integer and a floating point
    let p1 = Point { x: 5, y: 2.0 };
    // p2 has a string slice and a character
    let p2 = Point { x: "Hello", y: 'c' };
    // p3 will have the integer from p1 and the character from p2
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Note: not in book
    // This won't work, as p2 has different types for x and y than p1
    //let p4 = p1.mixupSameType(p2);
}
