pub trait Summary {
    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
    
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/* can also be written like this:
pub fn notify<T: Summary>(item: T) {

}
*/

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("elonmusk"),
        content: String::from("FREE AMERICA NOW!"),
        reply: false,
        retweet: false,
    };
}

fn main() {
    println!("*** Traits***");
    let tweet = Tweet {
        username: String::from("elonmusk"),
        content: String::from("Tesla stock price is too high imo"),
        reply: false,
        retweet: false,
    };

     notify(tweet);
}
