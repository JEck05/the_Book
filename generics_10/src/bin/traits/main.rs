mod traits;

use std::fmt::{Debug, Display};

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

    let new_tweet = Tweet{
        username: String::from("JamesEck"),
        content: String::from("hi, hello"),
        reply: false,
        retweet: false
    };


    println!("{}", new_tweet.summarize());

    println!("{}",news.summarize());
    
    notify(&new_tweet);
    
    notify(&news);
    
    let _x = some_function(&2,&3);
}
// Shows how a function can take in any parameter that implements a specific trait
// and then does trait operations(polymorphically)
fn notify(item: &impl Summary){
    println!("Breaking news {}", item.summarize());
}
// this is same as above just more verbose
//fn notify<T: Summary> (item: &T){}

// this more verbose way can be useful if we want to specify that
// two parameters must be the same type and implement Summary
// fn notify<T: Summary> (item1: &T, item2: &T){}

// if we want to specify that two parameters can be 2 different types
// and must implement summary we can do it this way
// fn notify(item1: &impl Summary, item2: &impl Summary){}


// this is an alternate way to write trait bounds
// it is useful because having many trait bounds can make
// the function signature hard to read
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug + Display,
{ print!("{}{}", t, u); 3 }
