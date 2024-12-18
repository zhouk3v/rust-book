use blog_types::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    // Note that request_review() and approve() return new struct instances
    // Use shadowing to save the returned instances
    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
