fn main() {
    //
    // String Slices
    //

    // String slices are references to part of a String (heap-allocated, not the stack-allocated string literal)
    /*
    let s = String::from("hello world");

    // Create slices by specifying the reference to the string (can be a string object, a reference to a string object (which includes other slices), or a string literal), followed by the range with the starting index (inclusive) and ending index (non-inclusive)
    
    // Creating slices from a string object
    let hello = &s[0..5];
    let world = &s[6..11];

    // Creating a slice from a reference to a string object
    let s_r = &s;
    let hello_2 = &s_r[0..5];

    // Creating a slice from another slice
    let he = &hello[0..2];

    println!("{hello} {world} {hello_2} {he}");
    */

    // You can drop the first index to include the first char of the string, or drop the last index to include the last char of the string

    /*
    // These two slices are the same (he)
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
    */

    /*
    // These two slices are the same (llo)
    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
    */

    /*
    // These two slices are the same (hello)
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
    */

    // A function that returns the first word of a string as a slice
    /*
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
    
        // Note: iter will allow us to traverse each letter in a string with a for loop
        // enumerate will wrap each letter in a tuple with the letter and an index
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
    
        &s[..]
    }
    // Using slices will give us an result that directly tied to the passed in string
    // This prevents against bugs related to situations where the passed in string might change, rendering the result out of date
    {
        let mut s = String::from("hello world");
    
        // word is an immutable reference, as first_word will return a slice, which is a immutable reference at i
        let word = first_word(&s);
    
        // Won't compile, since clear() will create a mutable reference when an immutable reference still exists
        s.clear();
    
        // Immutable reference to s is used here, and so it still exists when we try to use clear()
        println!("the first word is: {}", word);
    }
    */

    //
    // String literals as Slices
    //

    // string literals are of type &str, since it is a slice that points to that location in the binary (string literals are stored in the binary)
    // This also makes string literals are immutable - the &str type is immutable
    /*
    let s = "Hello, world!";
    */

    //
    // String slices as parameters
    //

    // When a function needs to accept strings as a parameter, use the '&str' type in the signature
    // This ensures that the function can accept references/slices to string objects and string literals (as they are also slices!)
    /*
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
    
        // Note: iter will allow us to traverse each letter in a string with a for loop
        // enumerate will wrap each letter in a tuple with the letter and an index
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
    
        &s[..]
    }

    {
        let my_string = String::from("hello world");
    
        // `first_word` works on slices of `String`s, whether partial or whole
        let word = first_word(&my_string[0..6]);
        let word = first_word(&my_string[..]);
        // `first_word` also works on references to `String`s, which are equivalent
        // to whole slices of `String`s
        let word = first_word(&my_string);
    
        let my_string_literal = "hello world";
    
        // `first_word` works on slices of string literals, whether partial or whole
        let word = first_word(&my_string_literal[0..6]);
        let word = first_word(&my_string_literal[..]);
    
        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word = first_word(my_string_literal);
    }
    */

    //
    // Other slices
    //

    // Arrays can also be sliced
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}
