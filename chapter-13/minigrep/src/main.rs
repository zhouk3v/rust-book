use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Pass in the iterator from env::args() into build() (build() will take ownership of the iterator)
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // The eprintln! macro prints to the standard error stream
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
