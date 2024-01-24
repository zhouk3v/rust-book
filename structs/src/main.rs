fn main() {
    /*
    // Structs are similar to tuples in which they can hold related values of different types, but now, those values are named (fields)

    // Defining a struct
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // Creating an instance of a struct
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // To access a value in a struct instance, use dot notation
    let email = user1.email;
    println!("{email}");

    // If the struct is mutable, we can modify the values in the struct
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let email = user1.email;
    println!("{email}");
     */
    
    //
    // Using the Field Init Shorthand
    //

    /*
    // Example function to create a user with pre-defined email and username
    fn build_user(email: String, username: String) -> User {
        // We can use Field Init shorthand when using variables with the same name as fields when creating an instance of a struct
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
     */

    //
    // Creating Instances from Other Instances with Struct Update Syntax
    //

    /*
    // the .. syntax will set fields that were not explicitly defined when creating an instance of an object to the values of a previously created object
    // NOTE: This WILL move any heap-allocated values (if any), which will invalidate the previous object. Both instances will still be valid if only stack-allocated values are set with ..
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // user1 is invalid after creating user2, so the below line won't work
    // println!("{}", user1.email);
    */

    //
    // Using Tuple Structs Without Named Fields to Create Different Types
    //

    /* 
    // Tuple structs are structs but without names for their fields. They are essentially named tuples

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Access fields of tuple structs with dot notation and indices
    let r = black.0;
    println!("{r}");
    // We can also destructure struct tuples similar to regular tuples
    let Point(x,y,z) = origin;
    println!("{x} {y} {z}");
    */

    //
    // Unit-Like Structs Without Any Fields
    //

    /*
    // Unit-like structs are structs without any fields and behave similarily to the unit type ()

    struct AlwaysEqual;
    let subject = AlwaysEqual;
    */

    //
    // Borrowing fields of a struct
    //

    // The borrow checker will track ownership at the struct level (the struct itself) and field level
    struct Point {x: i32, y: i32}

    let mut p = Point { x : 0, y: 0 };
    let x = &mut p.x; // At this point, p and p.x loses read, write and own permissions (but not p.y)


    // println!("{}", p.x); // Won't work, as p.x is being borrowed
    println!("{}", p.y); // Will still work, as p.y is not being borrowed
    *x += 1;

    println!("{}, {}", p.x, p.y);

}
