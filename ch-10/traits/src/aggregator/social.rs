// SocialPost implementation.

use super::Summary;


pub struct SocialPost {
    pub username: String,
    pub content:  String,
    pub reply:    bool,
    pub repost:   bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // uses default implementation of summarize()
}
