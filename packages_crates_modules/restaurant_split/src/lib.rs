// 
// Seperating Modules into different files
//

// This mod declaration (; instead of a {} body) will tell the compiler to look for a src/front_of_house.rs file
// Note: we only need to load a file once in the module tree. Other files will load the code from the file using a path from where the module was declared
// (i.e. to use the front_of_house module, other files will use the crate::front_of_house.. path)
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}