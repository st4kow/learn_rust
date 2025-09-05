//Here the type of structs are encoding the states
// Benfit: Compilation error if it is in a wrong state

pub struct Post {
    content: String
}
impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new()
        }
    }
    pub fn content(&self) -> &str {
        &self.content
   }
}

pub struct DraftPost {
    content: String
}
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }

    pub fn request_review(self) -> PendingReviewPost {
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
}