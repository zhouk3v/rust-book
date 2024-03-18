fn main() {
    //
    // Creating a new String
    //

    // Use String::new(); to create an empty string
    let mut s = String::new();

    // Use to_string() on a type that implements the Display trait to create a string with some initial data (eg string literals)
    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a string literal directly
    let s = "initial contents".to_string();

    // String::from() also creates a String from a string literal
    let s = String::from("initial contents");

    // Strings are UTF-8 encoded, all of the below are valid strings
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    //
    // Updating a string
    //

    // Use push_str to append a string slice
    let mut s = String::from("foo");
    println!("{}", s);
    s.push_str("bar");
    println!("{}", s);

    // The push_str method takes a string slice (equivelent to a reference to a String), since we don't want to take ownership of the parameter
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    // Still able to use s2 after passing it into push_str()
    println!("s2 is {s2}");

    // push() will append a single character to the string
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    //
    // Concatenation with the + Operator or the format! Macro
    //

    // The + operator can combine two existing strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // Note that the + operator will take ownership of the first argument, so s1 will no longer be valid after
    // The second argument is a reference, and so will still be valid (and accept either string literals or references to String objects)
    // The reason behind taking ownership of the first argument is that there might be a need to reallocate memory to append the second argument
    let s3 = s1 + &s2;

    // For more complex string appending, use the format!() macro
    // format!() will use references for its args and thus will not take ownership
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    // s1, s2, s3 are still valid here

    //
    // Indexing into Strings
    //

    // Indexing with [] is not supported by strings
    let s1 = String::from("hello");
    // Doesn't work
    //let h = s1[0];

    // At their core, strings are just arrays of bytes, but with the context of characters thrown in
    // The main reason is that Unicode chars take a variable amount of bytes to store. (E.g. english lettters take up 1 byte, but Cyrillc takes up 2 bytes)
    // Thus, an index in a string's bytes will not always correlate to a valid Unicode scalar value.
    // Anothe reason is that there are three relevant ways to look at strings (bytes, scalar values, and grapheme clusters). Rust provides different ways of interpreting raw string data
    // The last reason is that indexing are expected to take O(1) time, but that isn't possible to guarantee that performance with String

    //
    // Slicing strings
    //

    // As a possible alternative, use slices to index strings (i.e. use a range instead of a single digit with [])
    // Use range to create string slices with caution, again due to the variable length of Unicode chars

    // This will work
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    // This will panic
    // let s = &hello[0..1];

    //
    // Methods for iterating over strings
    //

    // Due to the various ways of looking at strings, Rust will make you be explict as if you want to go by characters or bytes

    // chars() iterates by Unicode characters (scalar values)
    for c in "Зд".chars() {
        println!("{c}");
    }

    // bytes() iterates by raw bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // grapheme clusters are not supported by the standard library
}
