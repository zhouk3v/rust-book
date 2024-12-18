//
// Note: due to the discontinous structure of the code for this chapter, the seperate sections of the chapter are not marked in the code
// It is recommended to go through the chapter itself instead
//

// A public `Post` struct
pub struct Post {
    // Post will hold a trait object of Box<dyn State> inside an Option<T> in a private `state` field
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // Associated public new() function to create an instance of Post
    pub fn new() -> Post {
        // Set the state of the new `Post` to a Some value that holds a Box
        // The Box points to a new instance of the Draft struct, to ensure that all new `Post` instances start as a draft
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        // Note that the add_text() function does not depend on the state the post is in, so it's not part of the state pattern
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // We want the value returned from content() to depend on the current state of the `Post`
        // Call a content() method on `state` and pass the post instance as an argument

        // The as_ref() method on Option<T> returns a reference to the value in Option<T> rather than ownership of the value (will return Option<&Box<dyn state>>)
        // Not using as_ref() will cause an error, as we can't move `state` out of the borrowed `&self`` of the function parameter

        // unwrap() is used here, since we know `state` is never None. `Post` will ensure that `state` will always contain a `Some` value

        // After unwrap(), content() is called, deref coercion will take care of the `&` and `Box<T>` from the &Box<dyn state>> returned from unwrap()
        self.state.as_ref().unwrap().content(self)
    }

    // Public method to transition a Post to the PendingReview state
    pub fn request_review(&mut self) {
        // If there is a state object, call the request_review() function on the state object
        if let Some(s) = self.state.take() {
            // The request_review() on the state object will consume the current state and return a new state object
            // To consume the old state, the request_review() method needs to take ownership of the state value.
            // This is why we use Option<T> in the `state`` field of `Post``, we call take() on the Option<T> to move the Some<T> value out of the state field and leave a None in its place
            // Note that Rust doesn't allow unpopulated fields in structs
            // We need to set `state` to `None` temporarily rather than setting it directly (as if there was no Option<T>) to get ownership of the `state` value
            // This ensures `Post` can't use the old `state` value after transforming it into a new state
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// Private `State` trait that define the behaviour that all state objects for a `Post` must have
trait State {
    // Internal request_review() method for State objects
    // Note that `self` is a Box<Self> rather than a `self`, `&self` or `&mut self`, which means the method is only valid on a Box holding the type
    // This syntax takes ownership of the Box<Self>, to invalidate the old state
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    // Internal approve() method for State objects
    // Similar to the above request_review() function
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // Internal content() method for State objects with default implementation that returns an empty string slice
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// Draft state struct
struct Draft {}

impl State for Draft {
    // Return a new, boxed instance of a PendingReview struct
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    // No effect
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // Return the same state object, since the post should stay in the PendingReview state if requested a review
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Return a new boxed instance of a Published struct
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    // No effect
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // No effect
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // Override the default implementation of content()
    // Return the `content` field on the passed in Post under a reference
    // Note the lifetime annotations: we're taking a reference to a `Post` and returning a reference to part of that `Post` (the lifetime of the returned reference is related to the `post` argument)
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
