// This is the crate root file (src/main.rs)

use crate::garden::vegetables::Asparagus;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
