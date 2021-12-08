pub trait Summary {
    fn summarize(&self) -> String;
    fn read_more(&self) -> String {
        String::from("(Read more...)")
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
    fn read_more(&self) -> String {
        String::from("(Read article...)")
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

// pub fn notify(item: impl Summary) {
//     println!("Breaking news! {}", item.summarize())
// }

pub fn notify<T: Summary>(item1: &T, item2: &T) {
    println!(
        "Breaking news! {}, and {}",
        item1.summarize(),
        item2.summarize()
    )
}

// pub fn my_notify(item: impl Summary + Display) {
// pub fn my_notify<T: Summary + Display>(item: T) {

fn main() {
    let my_tweet = Tweet {
        username: String::from("xfy"),
        content: String::from("xfy! xfy!! xfy!!!"),
        reply: false,
        retweet: false,
    };

    let my_article = NewsArticle {
        headline: String::from("XFY!"),
        location: String::from("Earth"),
        author: String::from("xfy"),
        content: String::from("xfy! xfy!! xfy!!! xfy! xfy!! xfy!!! xfy! xfy!! xfy!!!"),
    };

    println!(
        "1 new tweet: {} {}",
        my_tweet.summarize(),
        my_tweet.read_more()
    );

    println!(
        "1 new article: {} {}",
        my_article.summarize(),
        my_article.read_more()
    );

    notify(&my_article, &my_article);
}
