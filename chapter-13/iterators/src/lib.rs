//
// The `Iterator` Trait and the `next` method
//

// All iterators implement a trait named `Iterator` which is defined in the standard library

pub trait Iterator {
    // The `type` keyword represents an "associated type" with the trait
    type Item;

    // The iterator trait only requires implementors to define the `next` method
    fn next(&mut self) -> Option<Self::Item>;
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // We need to make the iterator mutable since calling next() changes the state of the iterator
    // This is known as consuming the iterator
    let mut v1_iter = v1.iter();

    // Note that the values from next are immutable references to the values in the vector
    // The iter() method produces an iterator over immutable references (Note that this immutably borrows the vector over the lifetime of the iterator)
    // The into_iter() method takes ownership of the caller, and returns owned values
    // The iter_mut() returns mutable references (Note that this mutably borrows the vector over the lifetime of the iterator)
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

//
// Methods that Consume the Iterator
//

// Methods that call next() are called consuming adaptors, since calling them uses up the iterator.
// One example is the sum() method, which takes ownership of the iterator and iterates through the items by calling next() repeatedly,
// thus, consuming the iterator.
// As sum() iterates through, it adds each item to a running total and returns the total when the iteration is complete

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    // We can't use v1_iter after sum() since sum takes ownership of the iterator
    assert_eq!(total, 6);
}

//
// Using Closures that Capture Their Environment
//

// Many iterator adapters take closures as arguments, and those closures most likely will capture their environment
// Note (not in book): Most of the iterator adapters will call the closure multiple times, usually up to the number of items in the iterator.
// This means that most iterators will only take a closure that implements the `FnMut` function trait

// Eg: the filter() method takes a closure which gets an item from the iterator and returns a bool
// If the closure returns `true`, the value will be included in the iterator produced by filter(),
// If the closure returns `false`, the value will not be included

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// Note: shoes_in_size takes ownership of the `shoes` vector
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // Remember that into_iter() takes ownership of the vector
    // Note: filter() will pass a reference to the value from the iterator to the closure inside
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                }
            ]
        );
    }
}
