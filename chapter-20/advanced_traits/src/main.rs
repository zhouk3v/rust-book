//
// Specifying Placeholder Types in Trait Definitions with Associated Types
//

// Associated types connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures
// The implementor of the trait will specify the concrete type to be used instead of the placeholder type for the particular implementation
// This is so we can define a trait that uses some type without needing to know exactly what those types are until the trait is implemented for a type

// Example: the `Iterator` trait
// The associated type is named `Item` and stands in for the type of the values the type implementing `Iterator` trait is iterating over
pub trait IteratorAssociatedType {
    // `Item` is a placeholder
    type Item;

    // The next() method's definition shows that it will return values of type `Option<Self::Item>`
    fn next(&mut self) -> Option<Self::Item>;
}
// Implementors of the `Iterator` trait will specify the concrete type for `Item`,
// and the next() method will return an `Option<T>` containing a value of that concrete type.

// Associated types seem similar to generics, where generics allow us to define a function without specifying what types it can handle
struct Counter;

impl IteratorAssociatedType for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

pub trait IteratorGeneric<T> {
    fn next(&mut self) -> Option<T>;
}

// The difference is that when using generics, we must annotate the types in each implementation,
// because we can also implement `Iterator<String>` for `Counter` or any other type
// Thus, we could have multiple implementations of `Iterator` for `Counter`.
// In other words, when a trait has a generic parameter, it can be implemented for a type multiple times,
// changing the concrete types of the generic type parameters each time
// When we use the next() method on `Counter`, we would have to provide type annotations to indicate which implementation of `Iterator` we want to use

// Not in book - examples of implementing the generic version of Iterator multiple times by changing the concrete type
impl IteratorGeneric<String> for Counter {
    fn next(&mut self) -> Option<String> {
        Some(String::from("Hello World!"))
    }
}

impl IteratorGeneric<i32> for Counter {
    fn next(&mut self) -> Option<i32> {
        Some(0)
    }
}

// With associated types, we don't need to annotate types because we can't implement a (non-generic) trait on a type multiple times
// We can only choose what the associated type will be once, because there can only be one `impl Iterator` for `Counter`
// We don't have to specify that we want an iterator of `u32` values everywhere that we call next() on `Counter`

// Associated types also become part of the trait's contract: Implementors of the trait must provide a type to stand in for the associated type placeholder
// Associated types often have a name that describes how the type will be used, documenting the associated type in the API documentation is good practice

//
// Default generic type parameters and operator overloading
//

// When using generic type parameters, we can specify a default concrete type for the generic type
// This eliminates the need for implementors of the trait to specify a concrete type if the default type works
// We use the `<PlaceholderType=ConcreteType>` syntax

// Example: operator overloading: customizing the behaviour of an operator (such as +)
// Rust doesn't allow the creation of your own operators or overload arbitary operators,
// but we can overload the operations and corresponding traits listed in `std::ops` by implementing the traits associated with the operator

// Example: overloading the `+` operator to add two `Point` instances together
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    // The associated type determines the type returned from the add() method
    type Output = Point;

    // The add() method adds the `x` values of two `Point` instances and the `y` values of two `Point` instances to create a new `Point`
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/*
fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    )
}
*/

// The default generic type in this code is within the `Add` trait
/*
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs:Rhs) -> Self::Output;
}
*/

// The `Rhs=Self` syntax is called "default type parameters"
// The `Rhs` generic type parameter defines the type of the `rhs` in the add() method
// If we don't specify a concrete type for `Rhs` when we implement the `Add` trait, the type of `Rhs` will default to `Self`,
// which will be the type that we're implementing `Add` on

// Example of implementing `Add` with a custom `Rhs` type

struct Millimeters(u32);
struct Meters(u32);

// Specify `impl Add<Meters>` to set the value of the `Rhs` type parameter instead of using the default of `Self`
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// There are two main ways to use default type parameters

// - To extend a type without breaking existing code
//     - This is if you want to add a type parameter to an existing trait, you can give it a default to allow extension of the functionality of the trait without breaking the existing implementation code
// - To allow customization in specific cases most users won't need
//     - This is similar to the `Add` trait example above

//
// Fully Qualifed Syntax for Disambiguation: Calling Method with the Same Name
//

// Note that in Rust, it is allowed for a trait to have a method with the same name as another method in a different trait,
// and for a type to implement both such traits
// We can also implement a method on a type directly with the same name as a method from a trait

// We need to tell Rust which method to use in the above cases

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

/*
fn main() {
    let person = Human;
    // The compiler will default to the method that is directly implemented on the type
    person.fly();

    // To call the fly() methods on the `Pilot` or `Wizard` trait, we need to use more explict syntax
    let person = Human;
    // Specifying the trait name before the method name clarifies to Rust which implementation of fly() we want to call
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
    // We could also write this to call the method directly implemented on `Human`, but it is more verbose
    Human::fly(&person);
}
*/

// Because methods takes a `self` parameter, if we had two types that both implement one trait,
// Rust could figure out which implementation of a trait to use based on the type of `self`

// Associated functions that are not methods do not have a `self` parameter
// When there are multiple types or traits that define non-methods with the same function name,
// Rust doesn't always know which type you mean unless "fully qualified syntax" is used

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    // Will default the baby_name() function that is directly implemented on `Dog`
    println!("A baby dog is called a {}", Dog::baby_name());
    // This doesn't work, Animal::baby_name() doesn't have a `self` parameter, and there could be other types that implement the `Animal` trait
    // Rust won't be able to figure out which implementation of Animal::baby_name() to call
    /*
    println!("A baby dog is called a {}", Animal::baby_name());
    */
    // To tell Rust which method to use, we need to use fully qualified syntax
    // We provide Rust with a type annotation within angle brackets
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

// In general, fully qualified syntax is defined as follows:
/*
<Type as Trait>::function(receiver_if_method, next_arg, ...);
*/

// For associated functions that are not methods, there would not be a `recevier`
// It is possible to use fully qualified syntax everywhere that you call functions or methods
// But, it is allowed to omit any part of this syntax that Rust can figure out from other information in the program
// You only need to use this syntax in cases where there are multiple implementations that use the same name and Rust needs help to identify which implementation to call.

//
// Using Supertraits to Require One Trait's Functionality Within Another Trait
//

// Sometimes, a custom trait definition will depend on another trait, in order to make use of the associated items of the second trait
// The trait that is being relied upon is called a supertrait of the trait that relies on it

// Ex: `OutlinePrint` trait, which uses the `Display` trait functionality in the outline_print() method

use std::fmt;

// We need to specify that the `OutlinePrint` trait will work only for types that also implement `Display`
// This can be done by specifying `OutlinePrint: Display`
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // We can use the to_string() function that is automatically implemented for types that implement `Display`
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// We need to implement `Display` on Point to implement `OutlinePrint` on Point
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

//
// Using the Newtype Pattern to Implement External Traits on External Types
//

// Remember that the orphan rule means that we're only allowed to implement a trait on a type if either the trait or the type are local to the crate
// It is possible to get around this restriction using the newtype pattern, which involves creating a new type in a tuple struct
// The tuple struct will have one field and be a thin wrapper around the type we want to implement a trait for
// Thus, the wrapper type will be local to our crate, so we will be able to implement traits on it

// Ex: implement `Display` on `Vec<T>`

// Use a `Wrapper` struct that holds an instance of `Vec<T>`
struct Wrapper(<Vec<String>);

// Implement `Display` on `Wrapper`
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The implementation of `Display` uses `self.0` to access the inner `Vec<T>`
        write!(f, "[{}]", self.0.join(", "))
    }
}

// The downside of using the Newtype pattern is that `Wrapper` is a new type, so it doesn't have the methods of the value it is holding
// If we wanted the new type to have every method the inner type has, implement the `Deref` trait on the `Wrapper` to return the inner type
// If we don't want the new type to have all the methods of the inner type, implement the methods we want manually