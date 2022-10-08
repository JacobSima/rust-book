#![allow(unused)]
use::aggregator::{
  Summary, 
  Tweet, 
  NewsArticleTwo, 
  SummaryTwo, 
  TweetTwo,
  SummaryThree
};   // library

mod trait_as_paremeter;

fn main() {
  
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know people,"),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize());

  let article = NewsArticleTwo {
    headline: String::from("Penguins win the Stanley Cup Championship"),
    location: String::from("Pittsburg, Pa, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
      "The Pittburgh Penguins once again are the best \
      hockey team in the HHL",
    )
  };

  println!("New article available! {}", article.summarize());

  let tweet_two = TweetTwo {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}\n", tweet_two.summarize());

  trait_as_paremeter::run();
}
