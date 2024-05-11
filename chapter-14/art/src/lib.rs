//
// Exporting a Covenient Public API with `pub use`
//

// If the crate has a large module hierarchy, it might be hard to use for outside users
// We can re-export items to make a public structure that is different from the internal structure by using `pub use`
// Re-exporting takes a public item in one location and makes it public in another location

//! # Art
//!
//! A library for modeling artistic concepts

// The generated API docs will now list and link re-exports on the front page
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
