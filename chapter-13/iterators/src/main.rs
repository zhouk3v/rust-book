fn main() {
    // Iterators are responsible for the logic of iterating over each item and determining when the sequence has finished

    // iterators are lazy - they have no effect until methods that consume the iterator are called
    let v1 = vec![1, 2, 3];

    // Iterator is stored in this variable
    let v1_iter = v1.iter();

    // Using the iterator in a for loop
    // When the for loop is called using an iterator, each element in the iterator is used in one iteration of the loop
    // Note that the for loop takes ownership of the iterator here, so we can't use v1_iter after
    for val in v1_iter {
        println!("Got: {}", val);
    }

    //
    // Methods that Produce Other Iterators
    //

    // Iterator adaptors are methods that produce different iterators by changing some aspect of the original iterator

    // Example: map() is an iterator adaptor that takes a closure to call on each item as the items are iterated through
    // map() will return a new iterator that produces the modified items

    let v1: Vec<i32> = vec![1,2,3];

    v1.iter().map(|x| x + 1);
    // Note that this code produces a warning for an unused iterator
    // This is because iterators are lazy, and don't do anything until consumed

    // We can use the collect() method to consume the iterator and return the items in a collection data type
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    // Since map() takes a closure, we can specify any operation we want to perform on each item,
    // which is an example of how closures allow customization of some behaviour,
    // while still keep the common behaviour of the Iterator trait

    // It is possible to chain multiple iterator adaptors, but a consuming adaptor needs to be called at the end to get the results
}
