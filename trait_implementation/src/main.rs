pub trait Summary {
    fn summarize(&self) -> String;
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

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let news_article = NewsArticle {
        headline: String::from("Amazing Scenes"),
        location: String::from("@ Laptop"),
        author: String::from("Eats Indigo"),
        content: String::from("Janky developer learns to print"),
    };

    println!("News Article Summary: {}", news_article.summarize());

    let tweet = Tweet {
        username: String::from("@EatsIndigo"),
        content: String::from("Beep boop"),
        reply: false,
        retweet: false,
    };

    println!("Tweet Summary: {}", tweet.summarize());
}
