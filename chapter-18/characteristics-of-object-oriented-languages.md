OOP languages share common characteristics, namely objects, encapsulation, and inheritance.

# Objects Contain Data and Behaviour

One definition of object-oriented programs is that the program is made up of objects, which packages both data and procedures (methods) that operate on that data.

By this definition, Rust is object-oriented: structs and enums have data, and `impl` blocks provide methods for both.

# Encapsulation that Hides Implementation Details

Encapsulation means that the implementation details of an object aren't accessible to code using that object. Therefore the only way to interact with an object is through its public API. Code using the object shouldn't be able to reach into the object's internals and change data or behaviour directly. This enables programmers to change and refactor an object's internals without needing to change the code that uses the object.

In Rust, we can use the `pub` keyword to decide which modules, types, functions, and methods in our code should be made public. By default, everything is private.

Example: an `AveragedCollection` struct that contains a vector of `i32` values. It can also have a field that contains the average of the values in the vector:

```Rust
pub struct AveragedCollection {
  list: Vec<i32>,
  average: f64,
}
```

The struct is marked `pub` so other code can use it, but the fields remain private. This is to ensure that whenever a value is added or removed, the average is also updated. We can implement methods to do this:

```Rust
impl AveragedCollection {
  pub fn add(&mut self, value: i32) {
    self.list.push(value);
    self.update_average();
  }

  pub fn remove(&mut self) -> Option<i32> {
    let result = self.list.pop();
    match result {
      Some(value) => {
        self.update_average();
        Some(value)
      }
      None => None,
    }
  }

  pub fn average(&self) -> f64 {
    self.average
  }

  fn update_average(&mut self) {
    let total: i32 = self.list().iter().sum();
    self.average = total as f64 / self.list.len() as f64;
  }
}

```

The public methods `add()`, `remove()`, and `average()` are the only ways to access or modify data in an instance of `AveragedCollection`.

The `add()` and `remove()` methods each call the private `update_average()` which handles updating the `average` field.

The `list` and `average` fields are private so that external code cannot add or remove items to or from `list` directly, or else the `average` field might become out of sync. The `average()` method returns the value in `average`, allowing external code to read it, but not modify it.

Because the the implementation details of `AveragedCollection` are encapsulated, we can easily change aspects, such as the data structure. As long as the signatures for `add()`, `remove()`, and `average()` public methods stay the same, code using `AveragedCollection` wouldn't need to change. If `list` was public, this wouldn't necessarily be the case, since adding and removing elements differ from data structure to data structure.

If encapsulation is a required aspect of a language to be considered object-oriented, then Rust meets that requirement via the usage of `pub`

# Inheritance as a Type System and as Code Sharing

Inheritance is a mechanism whereby an object can inherit elements from another object's definition, thus gaining the parent object's data and behaviour without the need of defining them again.

Rust does not have inheritance, there is no way to define a struct that inherits the parent struct's fields and method implementations without a macro.

There are two reasons for inheritance:

Reuse of code: you can implement behaviour for one type, and inheritance enables you to reuse that implementation for a different type. Rust provides a limited way using default trait method implementations, which can be overriden, similar to how a child class can override the method implementation inherited from a parent class.

Typing: Inheritance allows a child type to be used in the same places as the parent type. This is also known as polymorphism. This means you can subsitute multiple objects for each other at runtime if they share certain characteristics.

Inheritance has recently fallen out of favour, since it is often at risk of sharing more code than necessary. Subclasses shouldn't always share all characteristics of their parent class but will do so with inheritance. This can make a program's design less flexible. It also opens the possibility of called methods on subclasses that don't make sense or that cause errors. Some languages will only allow single inheritance, futher restricting design.

As a result, Rust uses the approach of trait objects instead of inheritance.
