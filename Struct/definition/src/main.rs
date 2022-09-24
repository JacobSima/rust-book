
#![allow(unused)]
/*
  We used the owned String type rather than the &str string slice type
  This will be discussed in the lifetimes in chapter 10
  Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is
    struct User {
      active: bool,
      username: &str,
      email: &str,
      sign_in_count: u64,
    }
*/


struct User{
  active: bool,
  username: String,
  email: String,
  sing_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;   // More use to create a trait of something

fn main() {
  let mut user_1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sing_in_count: 1,
  };

  // get struct value
  println!("\nUsername is: {}", user_1.username);
  println!("Email address is: {}\n", user_1.email);

  // change struct field
  // Note that the entire instance must be mutable; 
  // Rust doesn’t allow us to mark only certain fields as mutable
  user_1.email = String::from("jacob@gmail.com");
  user_1.username = String::from("jacob");
  user_1.sing_in_count = 3;

  println!("\nUsername is: {}", user_1.username);
  println!("Email address is: {}", user_1.email);
  println!("Sing in count is: {}\n", user_1.sing_in_count);


  // Create User from function
  let username = String::from("Moses");
  let email = String::from("moses@gmail.com");
  let user_2 = buil_user(email, username);

  println!("\nUsername is: {}", user_2.username);
  println!("Email address is: {}", user_2.email);
  println!("Sing in count is: {}", user_2.sing_in_count);
  println!("active: {}\n", user_2.active);

  // Creating Instances From Other Instances With Struct Update Syntax
  // user_3 go same data value with user_2 except few, let create user_3 based on user_2
  // Use this instead of copying same fields and value from user_2
  let user_3 = User {
    email: String::from("phila@gmail.com"),
    ..user_2    //comes last, take the rest of the value from user_2
  };

  println!("\nUsername is: {}", user_3.username);
  println!("Email address is: {}", user_3.email);
  println!("Sing in count is: {}", user_3.sing_in_count);
  println!("active: {}\n", user_3.active);

  // We cannot use user_2 here, as the username String has being moved to user_3
  // the Move and copy(HEAP), and Stack-only copy rules applied here for owenrship as well.

  //******* Using Tuple Structs without Named Fields to Create Different Types
  let black = Color(0, 0, 0);
  let origin = Point(25, 0, 10);
  println!("black.0: {}, origin.2: {}", black.0, origin.2)


}

fn buil_user(email: String, username: String) -> User {
  User{
    email,    // using the field init shorthand
    username,
    active: true,
    sing_in_count: 10,
  }
}
