// Opt in to printing out debug info for the struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    /* 
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // use {:?} to print out the struct in debug format
    println!("rect1 is {:?}", rect1);
    println!("The area of the rectangle is {} square pixels", area(&rect1));
    */

    // The dbg! macro

    // The dbg! macro will take ownership of an expression and prints the value and line number of where the dbg! macro is called in the stderr stream
    let scale = 2;
    let rect1 = Rectangle {
        // dbg! will return ownership after, so width will still get the same value
        width: dbg!(30*scale),
        height: 50,
    };

    // We don't want dbg! to take ownership of the struct, so pass a reference to it
    dbg!(&rect1);
}

// Note: we want to borrow the struct in order to be able to use it later
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}