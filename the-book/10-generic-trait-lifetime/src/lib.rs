use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
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

    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

pub fn notify(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify1<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary + Display, U: Clone + Debug>(a: T, _b: U) -> String {
    format!("Breaking new! {}", a.summarize())
}

pub fn notify3<T, U>(a: T, _b: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("Breaking new! {}", a.summarize())
}
struct _Pair<T> {
    x: T,
    y: T,
}

impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> _Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}