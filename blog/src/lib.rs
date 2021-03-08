// https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html
// This implements the State OOP design pattern for Rust
// But as described in the chapter, this is not really a good way to model this.

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        // unwrap won't panic because self.state is never None
        self.state.as_ref().unwrap().content(self)
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

// State trait supports transitioning between different states for the blog Post
trait State {
    fn get_state_name(&self) -> &str;
    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn get_state_name(&self) -> &str { "Draft" }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn get_state_name(&self) -> &str { "Pending Review" }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft{})
    }
}

struct Published {}

impl State for Published {
    fn get_state_name(&self) -> &str { "Published" }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draft_returns_empty_string() {
        let mut post = Post::new();
        assert_eq!("", post.content());

        // should still be empty even if we add content
        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        assert_eq!("Draft", post.state.unwrap().get_state_name());
    }

    #[test]
    fn test_pending_approval_returns_empty_string_still() {
        let mut post = Post::new();
        assert_eq!("", post.content());

        // should still be empty even if we add content and request review
        post.add_text("I ate a salad for lunch today");
        post.request_review();
        assert_eq!("", post.content());
        assert_eq!("Pending Review", post.state.unwrap().get_state_name());
    }

    #[test]
    fn test_approved_post_should_return_content() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        post.request_review();
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
        assert_eq!("Published", post.state.unwrap().get_state_name());
    }

    #[test]
    fn test_rejecting_moves_back_to_draft() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        post.request_review();
        post.reject();
        assert_eq!("", post.content());
        assert_eq!("Draft", post.state.unwrap().get_state_name());
    }
}