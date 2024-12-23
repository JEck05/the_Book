
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub trait Summary {
    fn summarize(&self) -> String{
        String::from("read more")
    }
}

impl Summary for NewsArticle {

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

fn main(){
    println!("hello");
    let news: NewsArticle = NewsArticle{
        headline: String::from("JamesEck"),
        content: String::from("..."),
        author: String::from("james"),
        location: String::from("fox news")
    };



    println!("{}",news.summarize());

}