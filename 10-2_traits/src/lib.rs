pub trait Summary {
    fn summarize(&self) -> String;

    // Default implementation
    fn can_reply_to(&self) -> bool {
        false
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn can_reply_to(&self) -> bool {
        true
    }
}

// item must implement the summary trait
pub fn nontify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
