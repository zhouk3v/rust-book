// Sometimes we want a library user to be able to extend the set of types that are valid in a particular situation
// E.g. In a GUI library, we can iterate through a list of items and call a draw() method on each of one to draw it onto a screen
// The GUI library can contain some types built-in, such as `Button` or `TextField`,
// but users might want to create their own types that can be drawn (e.g. `Image` and `SelectBox`)

// We won't be able to know and define all types that users want to create,
// but we do know that the GUI library will need to keep track of many values of different types, and it needs to call a draw() method on each of those types
// The library does not need to know what will happen when we call draw() on a type, just that the value will have the method defined

// Since Rust doesn't have inheritance (where we could just define a class/interface with a draw method() and then let other classes inhert/override it),
// we need another way to allow users to extend the library with new types

//
// Defining a Trait for Common Behaviour
//

// We defined a trait: `Draw` that will have one method: draw()
// We can define a vector that takes a `trait object`.
// A trait object points to both an instance of a type implementing the specified trait and a table used to look up trait methods on that type on runtime
// We create a trait object by specifying some sort of pointer (e.g. a & reference or a Box<T> smart pointer), then the `dyn` keyword, and then specifying the relevant trait
// We can use trait objects in place of a generic or concrete type.
// Wherever we use a trait object, Rust's type system will ensure at compile time that any value used in that context will implement the trait object's trait.
// As a result, we don't need to know all the possible types at compile time

// Trait objects are more like objects in other languages in the sense that they combine data and behaviour
// (where in Rust, they are usually seperated by structs and enums for data, and `impl` blocks for behaviour)
// However, trait objects differ from traditional objects in that we can't add data to a trait object.
// The specific purpose of trait objects is to allow abstraction across common behaviour

// Defining a trait: `Draw` with a draw() method
pub trait Draw {
    fn draw(&self);
}

// A `Screen` struct which holds a `components` vector that is of type `Box<dyn Draw>`, which is a trait object
// The trait object is a stand in for any type inside a Box that implements the Draw trait
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// Define a run() method which will call the draw() method on each item in `components`
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Note that this is different than defining a struct with a generic type parameter with trait bounds.
// A generic type parameter can only be substituted with one concrete type at a time.
// Trait objects allow for multiple concrete types to fill in for the trait object at runtime.

// As an example, this is the `Screen` struct with a generic type with trait bounds
/*
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
*/
// The `Screen` struct with generic types that have trait bounds is restricted to one type in `components` at a time.
// (e.g. If a `Button` is added into `components`, the rest of the components have to be `Button` too)
// Note that the generic types with trait bounds version is preferred for homogeneous collections, in order to take advantage of monomorphization at compile time

// In comparision, the `Screen` struct with trait objects can hold a combination of types.
// (e.g. `components` can hold a Box<Button>, and then a Box<TextField>)

//
// Implementing the trait
//

// A `Button` type that implements the Draw type, with fields for `width`, `height` and `label`
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a Button");
    }
}

// Note that the fields on `Button` will differ from the fields on other compnents
// (e.g. `TextField` might might have the same fields, plus a `placeholder` field)
// Each type that will be drawn will implement the `Draw` trait but use different code in the draw() method to define how to draw that particular type
// The `Button` type might have another `impl` block to define additional methods that won't apply to other types
