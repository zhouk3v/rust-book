// Another approach to the state pattern is to encode the states into different types

// Struct for a published post
pub struct Post {
    content: String,
}

// Struct for a draft post
pub struct DraftPost {
    content: String,
}

impl Post {
    // Create new posts with the Post::new() associated function
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    // Return the content of a published post
    pub fn content(&self) -> &str {
        &self.content
    }
}

// Note that DraftPost doesn't have a content() method, so any attempt to call content() on them will result in a compiler error
impl DraftPost {
    // Add text to the `content` field
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Return a PendingReviewPost struct
    // This consumes the DraftPost
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

// Struct for a Pending Review Post
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    // Return a Post struct
    // This consumes the PendingReviewPost
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
