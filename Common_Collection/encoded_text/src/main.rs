#![allow(unused)]

fn main() {
  // Create empty string literal
  let mut s = String::new();
  println!("s is: {s}");

  let data =  "initial contents";  // string slice
  let t =  data.to_string();    // convert slice string into string literal
  let u = "initial contents".to_string();
  let v =  String::from("initial contents");

  // updating a string, append with push_str and push
  s.push(' ');      // push a char
  s.push_str("of the book");    // append a string

  println!("s is: {s}");

  // Concatenation with +   fn add(self, s: &str) -> String {
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  let s4 = s1 + "-" + &s2 + "-" + &s3;    // s1 is out-of scope after this line
  println!("s4: {s4}");

  // Indexing into a string
  // let h = s1[0];   // String cannot be indexed by integer 
  // let i = &s1[0];  // String cannot be indexed by integer
  

  // slicing string
  let hello = "Здравствуйте";
  let z =  &hello[0..4];
  println!("z: {z}");

  let s5 =  &s4[0..3];
  println!("s5: {s5}");

  // Methods for interating over strings
  for c in s4.chars(){
    println!("{c}")
  }
  println!("\n");

  for b in s4.bytes()
  {
    println!("{b}")
  }

  // Strings Are Not So Simple in Rust
}
