pub struct Post {
    // INFO: in both Post and DraftPost content is a private field + there are no `state` field since we’re moving the encoding of the state to the types of the structs
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    // INFO: impossible to create a Post directly
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(), // making a draft with empty content
        }
    }

    // INFO: content is not accessible until we:
    // - call `request_review` on a DraftPost
    // - call `approve` on a PendingReviewPost
    // only then the post actually becomes a Post -> this method is accessible
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    // INFO: no content method in DraftPost since it's not a valid state
    // So that any attempt to get around these constraints will result in a compiler error.

    pub fn request_review(self) -> PendingReviewPost {
        // INFO: from DraftPost to PendingReviewPost
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
    // INFO: no content method defined
}
