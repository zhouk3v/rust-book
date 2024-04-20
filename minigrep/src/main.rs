use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // args() returns an iterator over the passed in command line args
    // collect() puts the elements of the iterator into a collection, depending on what the variable is annotated with
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        // The eprintln! macro prints to the standard error stream
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
