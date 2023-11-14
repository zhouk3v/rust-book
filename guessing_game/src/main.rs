use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // 1..=100 is a range expression
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        // Create a mutable variable and bind it to a new string
        let mut guess = String::new();

        // receive user input
        io::stdin()
            .read_line(&mut guess) // pass in a mutable reference to the guess variable
            .expect("Failed to read line");

        // Need to annotate the variable to tell Rust what type to convert to
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
