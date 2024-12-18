// This crates serves as solutions to the exercises under `Trade-offs of the State Pattern`

use blog_exercises::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    // Add a reject method that changes the post’s state from PendingReview back to Draft.
    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    // Require two calls to approve before the state can be changed to Published.
    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // Allow users to add text content only when a post is in the Draft state
    post.add_text(" and it was delicious");
    assert_eq!("I ate a salad for lunch today", post.content());
}
