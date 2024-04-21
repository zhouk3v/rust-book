use std::error::Error;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

/*
fn main() {
    //
    // Recoverable Errors with Result
    //

    /*
    // Result is an enum with two variants:
    // - Ok<T>
    // - Err<E>
    // The T and E are generic type parameters
    // - T is the type returned in a success case in the Ok variant
    // - E is the type returned in a failure case in the Err variant

    // One example of a function that returns a Result is File::open
    // In this case, T, E are prepopulated by the implementation of File::open
    // - T is the type std::fs::file, which will be a file handle
    // - E is the type std::io::Error, which will be an Err instance that will contain more info about the kind of error
    let greeting_file_result = File::open("hello.txt");
    // Use a match to handle the Result type from File::open
    let greeting_file = match greeting_file_result {
        // When the result is Ok, return the inner file value of the Ok variant, which will assign the file handle to greeting_file
        Ok(file) => file,
        // When the result is Err, panic
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    */

    //
    // Matching on Different Errors
    //

    /*
    // We can take different actions depending on the error
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // If we can't open the file, use a match statement on the kind() method to handle the error
        // The kind() method on the io:Error struct returns an enum variant
        Err(error) => match error.kind() {
            // If the file wasn't found, create the file using File::create(),
            // and use another match statement to handle the Result that is returned from File::create()
            ErrorKind::NotFound => match File::create("hello.txt") {
                // Return the file handler for the newly created file
                Ok(fc) => fc,
                // Panic if we can't create the file
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            // Panic if the error is something else (e.g. a permission error)
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // As an alternative, we can use unwrap_or_else() with a closure (more on this later)
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    })
     */

    //
    // Shortcut for Panic on Error: unwrap and expect
    //

    /*
    // The unwrap() shortcut function on the Result type will return the value in Ok if the Result is an Ok variant,
    // and call the panic! macro if the Result is an Err variant
    let greeting_file = File::open("hello.txt").unwrap();

    // The expect() shortcut function does the same as the unwrap() method, but allows customization of the panic message
    let greeting_file =
        File::open("hello.txt").expect("hello.text should be included in this project");
    */

    //
    // Propagating Errors
    //

    /*
    // If a function implementation calls something that might fail,
    // we can return the error to the calling code to decide what to do (return a Result type)
    // This is known as propagating the error
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");
        // Handle the Result type from File::open
        let mut username_file = match username_file_result {
            Ok(file) => file,
            // If we fail to open the file, return the error from File::open so the calling code can deal with it
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        // Read the file contents from the file handler with the read_to_string() function
        // Since read_to_string() can fail, it returns a Result type too
        match username_file.read_to_string(&mut username) {
            // Return the string if read_to_string succeeded
            Ok(_) => Ok(username),
            // Return the error from read_to_string
            Err(e) => Err(e),
        }
    }

    // The code that calls the read_username_from_file function will get a Result type, which it will need to handle
    let username_result = read_username_from_file();
    let username = match username_result {
        Ok(username) => println!("{}", username),
        // We can handle the error in a variety of ways here (panic, use a default username, etc)
        Err(e) => {
            let default = String::from("root");
            println!("{}", default)
        }
    };
     */

    //
    // A Shortcut for Propagating Errors: the ? Operator
    //

    /*
    fn read_username_from_file() -> Result<String, io::Error> {
        // The ? shortcut will act like a match statement
        // - If the operation before the ? succeeded (returned Ok<T>), the ? operator will return the value inside Ok<T>
        // - If the operation before the ? failed (returned Err<E>), the WHOLE function will exit early and return the Err<E> from the function
        // The ? operator will also call the from() function on the Err<E>, to convert it to the type of Error defined in the function signature
        // (provided that the Error type has implemented the from() function in an impl block)
        // This will allow us to define custom Error types to represent the variety of ways a function can fail
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    */

    /*
    // We can even chain multiple ? operators
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
    */

    // An even shorter implementation (which doesn't use ?)
    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }


}
 */

//
// Where The ? Operator Can Be Used
//

// The ? operator can only be used in functions that return a Result or Option value, more generally, any type that implements FromResidual
// - If used in an function that returns Option, the function will return None early if the ? operator returns None
fn last_char_of_first_line(text: &str) -> Option<char> {
    // lines() will return an iterator over the lines of the string (break on /n)
    text.lines().next()?.chars().last()
}
// Note that you must match the return type of the function
// In other words, you cannot use the ? operator on a function that returns Result within a function that returns Option,
// or use the ? operator on a function that returns Option within a function that returns Result
// As a workaround, you can use methods like ok() on Result or the ok_or() method on Option to convert between Option and Result explicitly

// main() can be modified to return a Result

// Box<dyn Error> in this case means "any kind of Error
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("Hello.txt")?;
    Ok(())
}

// When a main function returns a Result<(), E>, the executable will exit with a value of 0 if main returns Ok(())
// and will exit with a non-zero value if main returns with an Err value
// The main function may return any types that implement the std::process::Termination trait, which contains a function report() that returns an ExitCode
