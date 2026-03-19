pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        // INFO: starts with a Draft state and an empty content string
        Self {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // INFO: mut self since we're modifying the state
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // NOTE: as_ref() returns a reference to the value inside the Option rather than taking ownership -> Option<&Box<dyn State>> (not Option<Box<dyn State>>) is returned
        // NOTE: using unwrap() is safe here since the state is always Some during normal operation
        self.state.as_ref().unwrap().content(self)
        // INFO: so that the content is returned based on the current state
    }

    pub fn request_review(&mut self) {
        // INFO: changes the state to PendingReview
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review())
        }
    }

    pub fn approve(&mut self) {
        // INFO: changes the state to Published
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve())
        }
    }
}

impl Default for Post {
    fn default() -> Self {
        Self::new()
    }
}

trait State {
    // INFO: self:Box<Self> means the method is only valid when called on a Box holding the type
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // INFO: lifetime 'a is used to ensure the returned reference is valid as long as the Post is alive
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // INFO: content is not implemented for Draft, so it returns an empty string by default
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    // INFO: content is not implemented for PendingReview, so it returns an empty string by default
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        // INFO: content is implemented for Published, so it returns the post's content
        &post.content
    }
}
