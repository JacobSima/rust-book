#![allow(unused)]
use std::{env,fs,error::{Error} }; 

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ingore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
      if args.len() < 3 {
        return Err("not enough arguments");
      }
      let query =  args[1].clone();
      let file_path =  args[2].clone();
      let ingore_case = env::var("IGNORE_CASE").is_ok();
      Ok(Config { query, file_path, ingore_case})
    }
}

// Box<dyn Error> : return a type that implements the Error trait, but we don’t have 
// to specify what particular type the return value will be.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;

  // println!("With text:\n{contents}");
  // println!("\n{contents}");

  let result = if config.ingore_case {
    search_case_insensitive(&config.query, &contents)
  }else{
    search_case_sensitive(&config.query, &contents)
  };

  for line in result{
    println!("{line}");
  }

  Ok(())
}


pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
  // vec![]
  let mut results = Vec::new();

  for line in contents.lines(){
    if line.contains(query){
      results.push(line);
    }
  }

  results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines(){
    if line.to_lowercase().contains(&query){
      results.push(line);
    }
  }

  results
}


/*
**** TDD  : Test-driven developmen
1. Write a test that fails and run it to make sure it fails for the reason you expect.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed and make sure the tests continue to pass.
4. Repeat from step 1!
cargo test -- --show-output : view print output
IGNORE_CASE=1 cargo run -- to poem.txt : search_case_insensitive
cargo run -- to poem.txt : search_case_sensitive
cargo run > output.txt : The > syntax tells the shell to write the contents of 
standard output to output.txt instead of the screen.(But causes error now as not enough argument)

cargo run -- to poem.txt > output.txt : redirect printl! output to this file
*/
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive(){
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
  }

  #[test]
  fn case_insensitive(){
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    assert_eq!(
      vec!["Rust:","Trust me."],
      search_case_insensitive(query, contents)
    )
  }
}