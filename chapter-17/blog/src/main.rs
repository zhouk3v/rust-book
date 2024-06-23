// The `state pattern` is an OO design pattern
// The crux of the pattern is defining a set of states a value can have internally
// The states are represented by a set of `state objects`, and the value's behaviour changes based on its state

// The state objects share functionality through structs and traits in Rust (rather than objects and inheritance)
// Each state object is responsible for its own behaviour and for governing when it should change into another state.
// The value that holds a state object knows nothing about the different behaviour of the states or when to transition between states

// The advantage of the state pattern is that when the business requirements of the program change,
// we won't need to change the code of the value holding the state or the code that uses the value.
// We only need to update the code inside of the state objects to change its rules or add more state objects

// The example being implemented is a blog post workflow with the following functionality:
// - A blog post starts as an empty draft
// - When the draft is done, a review of the post is requested
// - When the post is approved, it gets published
// - Only published blog posts return content to print, so unapproved posts can't be accidentally be published
// - Any other changes attempted on a psot should have no effect, (e.g. approving a draft post before requesting a review will not work, the post will still remain a draft)

use blog::Post;

fn main() {
    // We want to allow the user to create a new draft blog post with Post::new()
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    // No text should be returned for draft posts
    assert_eq!("", post.content());

    // Enable a review for the post
    post.request_review();
    // No text should be returned when a post is awaiting review
    assert_eq!("", post.content());

    // Approve the post under review
    post.approve();
    // Published posts should return text
    assert_eq!("I ate a salad for lunch today", post.content());
}
