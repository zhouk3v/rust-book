use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // args() returns an iterator over the passed in command line args
    // collect() puts the elements of the iterator into a collection, depending on what the variable is annotated with
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Note that the program name takes up the first value of the args vector
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
