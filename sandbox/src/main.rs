//
// Fibonnaci
//

fn fibonnaci(num: u32)-> u32 {
    if num == 0 {
        return 0;
    }
    if num == 1 {
        return 1;
    }
    return fibonnaci(num - 1) + fibonnaci(num - 2)
}

fn main() {
    println!("{}", fibonnaci(10))
}