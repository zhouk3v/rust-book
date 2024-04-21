use std::fmt::Display;

fn main() {
    // Recall that every reference in Rust has a lifetime - the scope for which that reference is valid
    // Most of the time, lifetimes are implicit and inferred
    // We must annotate lifetimes when the lifetimes of references could be related in a few different ways
    // Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will defintely be valid

    //
    // Preventing Dangling References with Lifetimes
    //

    // The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it's intended to reference

    // The below won't compile

    /*
    // r is declared here with no initial value
    let r;

    {
        // x is declared here in an inner scope
        let x = 5;
        // r is set as a reference to x
        r = &x;
    }
    // Data for x is dropped here

    // r is read here (undefined behaviour)
    println!("r: {}", r);
    */

    //
    // The Borrow Checker Ensures Data Outlives Its References
    //

    // The borrow checker will compare scopes to determine whether all borrows are valid

    /*
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
    */
    // The lifetime of r is annotated with 'a and the lifetime of x is annotated with 'b
    // The inner 'b block is much smaller than the outer 'a lifetime block.
    // Rust will compare the size of lifetimes at compile time, and sees that r with a lifetime of 'a refers to memory with a lifetime of 'b
    // The program is rejected because 'b is shorter than 'a
    // More generally, the subject of the reference doesn't live as long as the reference

    // The below code is the fixed version
    /*
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
    */
    // x has the lifetime 'b, which is larger than 'a.
    // This means r can reference x since Rust knows that the reference in r will always be valid while x is valid.

    //
    // Generic Lifetimes in Functions
    //

    // Some code that calls the 'longest' function to print out the longest string
    /*
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    */

    // A non-compile longest() function
    // The function takes in string slices (which are references) to avoid taking ownership of the parameters
    /*
    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    */
    // The above function will not compile, with the compiler complaining about generic lifetime parameter
    // Here, Rust can't tell whether the reference being returned refers to x or y
    // We don't know either too, due to the if else block
    // When defining this function, we don't know the concrete valus that will be passed into this function, so we don't know whether the if case or the else case will execute
    // We also don't know the concrete lifetimes of the references that will be passed in, so we can't look at the scopes of the lifetimes to determine if the reference we return will always be valid
    // The borrow checker can't determine this either, because it doesn't know how the lifetimes of x and y relate to the lifetime of the return value

    //
    // Lifetime Annotation Syntax
    //

    // Lifetime Annotation DO NOT change how long any of the references live.
    // Rather, they descibe the relationship of the lifetimes of multiple references to each other
    // Functions can accept references with any lifetime by specifying a generic lifetime parameter

    // A lifetime annotation starts with an apostrophe ('), and then the name (which are usually all lowercase and very short)
    // The annotation is placed after the & of a reference

    /*
    &i32 // a reference
    &'a i32 // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime
    */

    //
    // Lifetime Annotations in Function Signatures
    //

    // To use lifetime annotations in function signatures,
    // we need to declare the generic lifetime parameters inside angle brackets between the function name and the parameter list

    // We want the signature to express the following constaint: the returned reference will be valid as long as both the parameters are valid.
    // This is the relationship between lifetimes of the parameters and the return value.

    // The fixed longest function
    /*
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    */

    // The function signature for longest() tells Rust that for some lifetime 'a, the function takes two parameters,
    // both of which are string slices that live at least as long as lifetime 'a.
    // The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a
    // In practice, it means that the lifetime of the reference returned by the longest() function is the same as the smaller of the lifetimes of the values referred to by the function arguments
    // We are specifiying that the borrow checker should reject any values that don't adhere to the above constraints
    // Note that the longest() function doesn't need to know exactly how long x and y will live, only that some scope (data lifetime) can be substituted for 'a that will satisfy this signature

    // The lifetime annotations go in the function signature, not in the function body
    // The annotations become part of the contract of the function, making the analysis for the compiler simpler.
    // If there's a problem with the way the function is written or called, the compiler errors can be more precise where the error is.

    // When we pass concrete references to longest(),
    // the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y.
    // In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y
    // Because we annotated the returned reference with the same lifetime parameter 'a,
    // the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.

    // Example of calling longest() with references that have different lifetimes (works)
    /*
    {
        let string1 = String::from("long string is long");
        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        } // data associated with  string2 and result is invalid here
    } // data associated with string1 is invalid here
    */

    // Example that doesn't work
    /*
    {
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        } // data associated with string2 and result are invalid here
        println!("The longest string is {}", result); // result being used here, but it might be pointing to freed data
    }
    */
    // Even though the longest() function will return a reference to string1, which will still be valid,
    // the compiler is told that the lifetime of the returned reference (result) will be the same as the lifetime of the reference of the string2 variable,
    // which is already invalid after the inner scope when the result reference is used again

    //
    // Thinking in Terms of Lifetimes
    //

    // The way that lifetime parameters need to be specified depends on what the function is doing

    // Example: if longest only returned the first argument (x), the second argument (y) doesn't need a lifetime specifier
    /*
    fn longest<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }
    */

    // Note: we can still have a lifetime parameter on y in this case, which can be the same or different:
    /*
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        x
    }
    */

    fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
        x
    }

    // This is because the lifetime of y is not related to the lifetime of x, or the returned reference

    // When returning a reference from a function,
    // the lifetime parameter for the return type needs to match the lifetime parameter for one of the function parameter
    // If the reference does not refer to one of the parameters, it must refer to a value created within the function
    // However, this would result in a dangling reference, as the value created within the function will be dropped at the end of the function
    /*
    fn longest<'a>(x: &str, y: &str) -> &'a str {
        let result = String::from("really long string");
        result.as_str()
    }
    */
    // In this case, the return value lifetime parameter is not related to the lifetime of the function parameters at all
    // The best fix here is to return an owned data type rather than a reference,
    // and let the function caller clean it up afterwards (which will happen when the caller goes out of scope)
    // (This will apply to any case where the return value of a function is a value that is solely generated within the function itself here)

    // The key idea of lifetime syntax (annotations) to make connections between the lifetimes of various parameters and return values of functions

    //
    // Lifetime Annotations in Struct Definitions
    //

    // We can define structs to hold references,
    // but in that case, we would need to add lifetime annotations on every reference in the struct's definitions
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    // Remember that iterators return a reference
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // Create an instance of ImportantExcerpt, which holds a reference to the first sentence of the String owned by `novel`.
    // The data in `novel` exists before the ImportantExcerpt instance is created, and doesn't go out of scope until after the ImportantExcerpt instance goes out of scope
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    //
    // Lifetime Elision
    //

    // Before, every function parameter that was a reference needed an explicit lifetime via a lifetime annotation
    fn first_word<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    // Now, there are patterns programmed in Rust's analysis of references to infer the lifetimes of common situations
    // These patterns are called the lifetime elision rules, which are a set of particular cases that the compiler will consider
    // The elision rules don't provide full inference. If there is still ambiguity after applying these rules,
    // the compiler will throw an error, which can be fixed with lifetime annotations

    // Lifetimes on function or method parameters are called `input lifetimes`, and lifetimes on return values are called `output lifetimes`

    // The compiler uses three rules to figure out the lifetimes of the references when there aren't explict annotations:

    // Rule 1: The compiler assigns a different lifetime parameter to each lifetime in each input type.
    // - References like &'_ i32 need a lifetime parameter
    // - Structures like ImportantExcerpt<'_> need a lifetime parameter
    // - Examples
    //   - The function `fn foo(x: &i32)` would be fn foo<'a>(x: &'a i32)
    //   - The function `fn foo(x: &i32, y: &i32)` would be `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`
    //   - The function `fn foo(x: &ImportantExcerpt)` would be `fn foo<'a, 'b>(x: &'a ImportantExcerpt<'b>)` (Note the seperate lifetime parameter for the reference inside ImportantExcerpt)

    // Rule 2: If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
    // - Example: fn foo<'a>(x: &'a i32) -> &'a i32

    // Rule 3: If there are multiple input lifetime parameters, but one of then is &self or &mut self (in this case, the function would be a method),
    // the lifetime of self is assigned to all output lifetime parameters.

    // If the compiler still finds references that don't have a lifetime associated with it after applying all three rules, it will throw an error

    // Example: the longest() function without lifetime annotations

    // Start
    // fn longest(x: &str, y: &str) -> &str { }

    // After applying the first rule
    // fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str { }

    // We can't apply the second rule since there is more than one input lifetime parameter
    // fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str { }

    // We can't apply the third rule since longest() doesn't have &self or &mut self in its parameters
    // fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str { }

    // Final
    // fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str { }
    // Since there is no lifetime annotation for the return parameter, the compiler will throw an error

    //
    // Lifetime Annotations in Method Definitions
    //

    // We use the same syntax as generic type parameters for methods on structs with lifetimes
    // Where we use lifetime parameters depends on whether they're related to the struct fileds or the method parameters and return values

    // Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct's name,
    // because those lifetimes are part of the struct's type.

    // In method signatures inside the impl block, references might be tied to the lifetime of references in the struct's fields, or they might be independent.
    // The lifetime elision rules often make it so that lifetime annotations aren't necessary in method signatures.

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
    // The lifetime parameter declaration after impl and its use after the type name are required,
    // but we're not required to annotate the lifetime of the reference to `self` because of the first elision rule.

    // Example where the third elision rule applies:
    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // There are two input lifetimes, so Rust will apply the first rule and give &self and announcement their own lifetimes.
    // Since one of the parameters is &self, the return type gets the lifetime of &self.

    //
    // The static lifetime
    //

    // References with the 'static lifetime can live for the entire duration of the program
    // All string literals have the 'static lifetime

    let s: &'static str = "I have a static lifetime";

    // The text of the string is stored in the program's binary, which is always available
    // Therefore, the lifetime of all string literals is 'static

    // The compiler error's messages might suggest to use the 'static lifetime for a reference
    // But think about if the reference actually lives for the entire program and if you want it to
    // Most of the time, an error that suggests using the 'static lifetime results from trying to create a dangling reference or a mismatch of the available lifetimes

    //
    // Generic Type Parameter, Trait Bounds and Lifetimes together:
    //

    // Example of a function that uses a generic type parameter, a trait bound and a lifetime
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // The `ann` argument has a generic type T, which can be filled with any type that implements the Display trait that is specified with the `where` clause
    // Because lifetimes are a type of generic,
    // the declarations of the lifetime parameter 'a and the generic type parameter T go in the same life inside the angle brackets after the function name
}
