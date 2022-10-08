// Traits are similar to a feature often called interfaces in other languages, 
// although with some differences.

// Defining a Trait
pub trait Summary {
  fn summarize(&self) -> String;
}

// Implementing a Trait on a Type
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

pub struct Tweet{
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
  }
}


// Default Implementations
pub trait SummaryTwo {
  fn summarize(&self) -> String {
    String::from("Read more...")
  }
}

pub struct NewsArticleTwo 
{
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl SummaryTwo for NewsArticleTwo {}


// Default implementations can call other methods in the same trait, 
// even if those other methods don’t have a default implementation
pub trait SummaryThree {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}

pub struct TweetTwo{
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool
}

impl SummaryThree for TweetTwo {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}