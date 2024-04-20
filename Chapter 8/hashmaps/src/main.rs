// Need to import the library first, unlike with vectors and strings
use std::collections::HashMap;

fn main() {
    //
    // Creating a new hashmap
    //

    // Use `new()` to create a new hashmap
    let mut scores = HashMap::new();

    // Add new keys to the Hashmap with `insert()`
    // All of the keys must have the same type, and all of the values must have the same type
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Hashmaps store their data on the heap

    //
    // Accessing Values in a Hash Map
    //

    // The `get()` method provides the value of the given key
    let team_name = String::from("Blue");
    // get() will return an Option<&T> (note the reference), if there is no value, it will return None
    // The following function uses copied() to copy the value of the reference (i32 is stack-allocated),
    // and then unwrap_or() to set `score` to 0 if scores doesn't have an entry for "Blue"
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // We can also iterate through a Hashmap with a for loop (which will return the key-value pairs in an arbitary order)
    for (key, value) in &scores {
        println!("{key} {value}");
    }

    //
    // Hash Maps and Ownership
    //

    // Types that implement the Copy trait will be copied into the HashMap.

    // For owned values (e.g. String), the values will be moved and the HashMap will be the new owner of the values

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are no longer valid
    //println!("{field_name} {field_value}");

    // If we insert references to values into the HashMap, then the values won't be moved into the HashMap
    // Note that the values that the references point to must be valid at least as long as the HashMap is valid.

    //
    // Updating a Hash Map
    //

    // Each unique key can only have one value associated with it at a time. (but values can be associated with multiple keys)

    // There are a few ways to handle the case where a key is already associated with a value in the HashMap

    // Overwriting a Value
    // Calling insert() on an existing key will overwrite the original value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    // Will print {"Blue": 25}
    println!("{:?}", scores);

    // Adding a Key and Value Only if a Key Isn't Present

    // entry() will check if the passed-in value already exists in the Hash Map
    // The or_insert() method on entry() will return a mutable reference to the value if the key in entry() exists,
    // If the key does not exist, it inserts the passed-in value as the new value for the key and returns a mutable reference to the new value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // This line will insert the "Yellow" key with the value 50 into the Hash Map
    scores.entry(String::from("Yellow")).or_insert(50);
    // This line will not change the Hash Map
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a Value Based on the Old Value

    // Remember that or_insert() will return a mutable reference to the value associated with the key in entry()

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // Side note that has nothing to do with Hash Maps: split_whitespace() will return an iterator over sub-slices, seperated by whitespace (equivelent to split() in python)
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // Dereference the mutable reference from or_insert()
        *count += 1;
    }

    println!("{:?}", map);
}
