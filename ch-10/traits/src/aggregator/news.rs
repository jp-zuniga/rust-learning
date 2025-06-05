// NewsArticle implementation.

use super::Summary;


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author:   String,
    pub content:  String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, {} ({})", self.headline, self.summarize_author(), self.location)
    }
}
