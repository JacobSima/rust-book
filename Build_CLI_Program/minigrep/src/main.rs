#![allow(unused)]
use std::{env, process,}; 
use minigrep::Config;
/*

std::env::args : returns an iterator of the command line arguments
Iterators produce a series of values, and we can call the collect method on an iterator 
to turn it into a collection, such as a vector

*/


fn main() {
  let args:Vec<String> = env::args().collect();
  // dbg!(args);
  let config =  Config::build(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}");
    process::exit(1);
  });

  // println!("Searching for: {}", config.query);
  // println!("In file: {}", config.file_path);

  // run(config).unwrap_or_else(|error| {
  //   println!("Should have been able to read the file");
  //   println!("Application error: {error}");
  //   process::exit(1);
  // });
  if let Err(e) = minigrep::run(config){
    eprintln!("Should have been able to read the file");
    eprintln!("Application error: {e}");
    process::exit(1);
  }
 
}



