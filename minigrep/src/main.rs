use std::env;

fn main() {
    // args() returns an iterator over the passed in command line args
    // collect() puts the elements of the iterator into a collection, depending on what the variable is annotated with
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    // Note that the program name takes up the first value of the args vector
    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
