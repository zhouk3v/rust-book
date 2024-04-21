use std::fmt::Debug;
use std::fmt::Display;
// Traits define functionality that is shared between types in an abstract way
// We can use trait bounds to specify that a generic type can be any type that has certain behaviour
// Think of traits as askin to interfaces in Java and Typescript

// Structs that implement the Summary trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//
// Defining a trait
//

// A type's behaviour is defined by the methods that we can call on that type
// Different types share the same behaviour if we can call the same methods on all of those types
// Trait definitions are a way to group together method signatures to define a set of behaviours necessary to accomplish some purpose

// A Summary trait (make it public to make it available to other crates)
// Declare a new trait with the `trait` keyword, followed by the name
pub trait Summary {
    // Define method signatures in the body of the trait
    // After the method signature, use a semicolon(;) instead of {}
    // Traits can have multiple method signatures
    // Each type implementing the trait must provide its own custom behaviour for the body of the method, this is enforced by the compiler

    // A summarize method signature that the types implement
    fn summarize(&self) -> String;
}

//
// Implementing a Trait on a Type
//
// Put the trait name after `impl`, then use the for keyword, then specify the name of the type that implements the trait (struct or enum)
impl Summary for NewsArticle {
    // For each method signature in the trait, use curly brackets and fill in the function body
    // to define the behaviour we want the methods of the trait to have for this particular type
    fn summarize(&self) -> String {
        format!("{}, {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Note that types defined elsewhere can also implement the Summary trait from this library crate if it's brought into scope
// This library crate can define external traits (such as ones in the standard library) on structs defined in this crate (e.g. define Display on Tweet)
// This library crate can also define the Summary trait onto standard library types (e.g. Vec<T>)
// More generally, a local type can implement an external trait, and a local trait can be implemented on an external type,
// but an external type cannot implement an external trait (i.e. Vec<T> cannot implement Display here)

//
// Default Implementations
//

/*
// We can specify default behaviour for a method in the trait, within the trait block itself
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// To use a default implementation for a method in the trait, don't define the method in the impl block
impl Summary for NewsArticle {}

// The default behaviour can be overriden in the impl block for a type
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
*/

// Default implementations can call other methods in the same trait, even if those other methods don't have a default implementation
/*
pub trait Summary {
    // summarize_author() doesn't have a default implementation, and so needs to be implemented by the type that implements Summary
    fn summarize_author(&self) -> String;

    // summarize() has a default implementation, which calls summarize_author()
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    // Tweet implements summarize_author as required
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // Tweet will use the default behaviour of summarize
}
 */
// Note that it isn't possible to call the default implementation from an overriding implementation of the same method

//
// Traits as Parameters
//

// We can use traits to define functions that accept many types (such as generics)
/*
// To use a trait as a parameter, use the impl keyword followed by the trait name, instead of a concrete type
// The notify function can be called with any type that implements the Summary trait under the `item` argument
pub fn notify(item: &impl Summary) {
    // We can call any methods in the Summary trait within notify()
    println!("Breaking news! {}", item.summarize());
}
*/

//
// Trait Bound syntax
//

// The impl Trait syntax is syntax sugar for a longer from called a Trait Bound
/*
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
*/

// The impl Trait syntax is convenient and makes for more consise code
// The fuller trait bound syntax can express more complex cases

// We can have mutliple parameters that implement Summary, as represented with the impl syntax
// This is fine if the parameters can be different types
/*
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
*/

// If the parameters need to be the same type, we must use a trait bound
// The generic type T specified as the type of item1 and item2 constrains the item1 and item2 arguments to be the same type
/*
pub fn notify<T: Summary>(item1: &T, item2: &T) {}
*/

//
// Specifiying Multiple Trait bounds with the + syntax
//

// We can use the + syntax to specify that an argument for a function needs to implement multiple traits
// item needs to implement both the Summary trait and the Display trait
//
/*
pub fn notify(item: &(impl Summary + Display)) {}
*/
// The + syntax can also be used for trait bounds on generic types
/*
pub fn notify<T: Summary + Display>(item: &T) {}
 */

//
// Clearer trait bounds with where clauses
//

// We can use `where` to clean up function signatures with multiple generic types that have multiple trait bounds

// This
/*
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
*/
// Is the same as this
/*
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}
*/

//
// Returning Types that Implement Traits
//

// Instead of a concrete type, we can use impl syntax to return a value of some type that implements the trait specificed by impl
// This is useful for closures and iterators (covered later)
/*
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
*/

// However, we are still limited to returning a single type from a function
// The below function that returns either a NewsArticle or a Tweet will not work
/*
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
*/

//
// Using Trait Bounds to Conditionally Implement Methods
//

// We can use trait bounds in an impl block that uses generic type parameters,
// in order to implement methods conditionally for types that implement the specified traits

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// This impl block will only implement its methods on an instance if the inner type 'T' of the instance implements the PartialOrd trait and the Display trait
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements another trait
// This is called a blacket implementation

// As a in-language example, the standard library implements the ToString trait on any type that implements the Display trait
impl<T: Display> ToString for T {}

// Because of this, we can call to_string() for any type that implements the Display trait
let s = 3.to_string();

