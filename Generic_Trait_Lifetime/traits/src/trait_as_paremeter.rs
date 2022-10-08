#![allow(unused)]
use std::fmt::Display;
use std::fmt::Debug;

use::aggregator::{
  Summary, 
  Tweet, 
  NewsArticleTwo, 
  SummaryTwo, 
  TweetTwo,
  SummaryThree
};

pub fn run(){
  
}

// function that argument anything that implement the Summary trait.
// function based on Summary trait behavior
// pub fn notify(item1: &impl Summary, item2: &impl Summary)
pub fn notify(item: &impl Summary){
  println!("Breaking news! {}", item.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax
// pub fn notify(item: &(impl Summary + Display))

// Clearer Trait Bounds with where Clauses
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
  23
}

// Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary {
  Tweet {
      username: String::from("horse_ebooks"),
      content: String::from(
          "of course, as you probably already know, people",
      ),
      reply: false,
      retweet: false,
  }
}