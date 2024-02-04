// Opt in to printing out debug info for the struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl (implementation) blocks will contain associated functions attached to the particular struct type
impl Rectangle {
    // The first parameter of a method is always &self, which is usually a reference to the caller of the struct (can be made mutable with &mut self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // We can define method with names that are the same as a struct's field
    fn width(&self) -> bool {
        self.width > 0
    }

    // Example of a method with multiple parameters (in this case, an immutable reference to another Rectangle struct)
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated functions (that aren't method) are functions in impl blocks that don't take self as a parameter
    // They are often used for constructors that will return a new instance of the struct
    // The Self keyword is an alias for the type that appears after impl
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // NOTE, because self is not a reference here, this method will take ownership of the caller!
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height)
        }
    }

    // This method won't compile, as the reference does not have permission to move the rect object into the max function
    // fn set_to_max(&mut self, other: Rectangle) {
    //     *self = self.max(other);
    // }

}

// We can have multiple impl blocks for a struct type
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // We call the method with the dot notation
    );

    // When we follow rect1.width with (), rust knows that we are calling a method
    if rect1.width() {
        // Whem we don't follow rect1.width with (), rust knows that we are refering the field
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // To call associated functions, use the :: syntax with the struct name
    let sq = Rectangle::square(3);

    // Method calls are syntatic sugar for associated function calls
    let mut r = Rectangle {
        width: 1,
        height: 2
    };
    // The below 2 function calls are equivelent
    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    assert_eq!(area1, area2);

    // The below 2 function calls are equivelent
    r.set_width(2);
    Rectangle::set_width(&mut r, 2);

    // Rust will take care of references/dereferences when using dot notation for methods
    let r = &mut Box::new(Rectangle {
        width: 1,
        height: 2
    });
    let area1 = r.area();
    // First dereference for the &mut (now Box<Rectangle>), second derefernce for the Box (now Rectangle), then add a reference again since area() expects a reference
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1, area2);

    //
    // Methods and ownership
    //

    // rect will have read and owned permissions
    let rect = Rectangle {
        width: 0,
        height: 0,
    };

    // We can call area and max on rect
    println!("{}", rect.area());

    let other_rect = Rectangle { width: 1, height: 1};

    let max_rect = rect.max(other_rect);

    // But we can't call set_width, since rect is missing the write permission (it was not declared with mut)
    // let rect = Rectangle {
    //     width: 0,
    //     height: 0,
    // };
    // rect.set_width(0);

    // A similar error will occur for non-mutable references to mutable objects
    let mut rect = Rectangle {
        width: 0,
        height: 0
    };

    // This works
    rect.set_width(1);

    let rect_ref = &rect;
    // This won't
    //rect_ref.set_width(2);

    // Calling methods with a 'self' parameter (as opposed to &self) will move the input struct
    let rect = Rectangle {
        width: 0,
        height: 0,
    };

    let other_rect = Rectangle {
        width: 1,
        height: 1
    };

    let max_rect = rect.max(other_rect);

    // Won't work, as rect lost read and owner permissions when max_rect was called (the underlying object on the heap was moved into the scope of max_rect)
    //println!("{}", rect.area());    
}