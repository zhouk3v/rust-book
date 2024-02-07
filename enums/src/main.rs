// enums are a custom data type that give us a way of saying a value is one of a possible set of values
enum IpAddrKind {
    V4,
    V6,
}

// We can define data to put into each enum variant (such as a string)
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// Each variant can hold different types and amounts of associated data (either primitive types to structs)
enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}

// Another example of an enum with various types in its variants
enum Message {
    Quit, // has no data associated with it
    Move { x: i32, y: i32}, // has named fields
    Write(String), // has a single string
    ChangeColor(i32, i32, i32), // has three i32 values
}

// We can define associated functions onto enums with impl

impl Message {
    fn call(&self) {

    }
}

fn main() {
    // Creating instances of the IpAddrKind variants
    // The variants are namespaced under the identifier
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Calling a function with an enum value
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Attaching data to an enum variant
    // The name of the enum variant becomes a function that constructs an instance of the enum
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    // Calling a function associated with an enum
    let m = Message::Write(String::from("Hello"));
    m.call();

    // The option enum is used as a replace for 'null' values in Rust
    // It has two variants - Some: which contains a value, and None, which represents null
    let some_number = Some(5);
    let some_char = Some('e');

    // Rust requires an annotation when setting a None value
    let absent_number: Option<i32> = None;

    // The below won't compile
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y; // x and y are different types (i8 and Option<i8>)

    // When dealing with Option<T> values, you need to explicitly handle the case where the value is null

}

// A function that takes in the IpAddrKind enum
fn route(ip_kind: IpAddrKind) {

}
