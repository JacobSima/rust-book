#![allow(unused)]
/*
recoverable error, such as a file not found error, we most likely just want 
to report the problem to the user and retry the operation 

enum Result<T, E> {
    Ok(T),
    Err(E),
}

*/

use core::panic;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::fs;

fn main() {
  // *** The return type of File::open is a Result<T, E>
  // let greeting_file_result = File::open("hello.txt");
  // let greeting_file = match greeting_file_result {
  //   Ok(file) => file,
  //   Err(error) => panic!("Problem opening the file: {:?}", error),
  // };



  // *** Matching on Different Errors
  // let greeting_file_result = File::open("hello.txt");
  // let greeting_file = match greeting_file_result {
  //   Ok(file) => file,
  //   Err(error) => match error.kind() {
  //       ErrorKind::NotFound => match File::create("hello.txt") {
  //           Ok(fc) => fc,
  //           Err(e) => panic!("Problem creating the file: {:?}",e),
  //       },
  //       other_error => panic!("Problem opening the file: {:?}", other_error),
  //   },
  // };
   


  // *** Alternatives to Using match with Result<T, E>
  // let greeting_file = File::open("hello.txt").unwrap_or_else(|error|{
  //   if error.kind() == ErrorKind::NotFound {
  //     File::create("hello.txt").unwrap_or_else(|error|{
  //       panic!("Problem creating the file: {:?}", error);
  //     })
  //   }else{
  //     panic!("Problem opening the file: {:?}", error);
  //   }
  // });


  // *** Shortcuts for Panic on Error: unwrap and expect
  /*
  If the Result value is the Ok variant, unwrap will return the value inside the Ok. 
  If the Result is the Err variant, unwrap will call the panic!
  */

  // let greeting_file = File::open("hello.txt").unwrap();

  // --- the expect method lets us also choose the panic! error message

  // let greeting_file = File::open("hello.txt")
  //     .expect("hello.txt should be included in this project");



 

  let c = last_char_of_first_line("mboka ebebi");
  println!("{}",c.unwrap());

}

/*
This is known as propagating the error and gives more control to the calling code, 
where there might be more information or logic that dictates how the error 
should be handled than what you have available in the context of your code.
*/
// *** Propagating Errors
fn read_username_from_file() -> Result<String, io::Error> {
  let username_file_result = File::open("hello.txt");

  let mut username_file = match username_file_result {
      Ok(file) => file,
      Err(e) => return Err(e)
  };

  let mut username = String::new();
  match username_file.read_to_string(&mut username) {
    Ok(_) => Ok(username),
    Err(e) => Err(e),
  }
}

/*
 If the value of the Result is an Ok, the value inside the Ok will get returned from 
 this expression, and the program will continue. If the value is an Err, 
 the Err will be returned from the whole function as if we had used the return keyword so 
 the error value gets propagated to the calling code.
*/
// *** A Shortcut for Propagating Errors: the ? Operator
fn read_username_from_file_2() -> Result<String, io::Error> {
  let mut username_file = File::open("hello.txt")?;
  let mut username = String::new();
  username_file.read_to_string(&mut username)?;
  Ok(username)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
  let mut username = String::new();
  File::open("hello.txt")?.read_to_string(&mut username)?;
  Ok(username)
}

/*
? operator in a function that returns 
Result, Option, or another type that implements FromResidual.
*/
fn last_char_of_first_line(text: &str) -> Option<char> {
  text.lines().next()?.chars().last()
}

fn read_username_from_file_1() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}
