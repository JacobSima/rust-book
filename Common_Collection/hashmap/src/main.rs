use std::collections::HashMap;

fn main() {
  // One way to create an empty hash map is using new and adding elements with insert
  // Hasmap saves data in the heap
  let mut scores =  HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  // Accessing values in HasMap
  let team_name = String::from("Blue");
  // get return an Option<&V>
  // copied return Option<i32>, rather than Option<&i32>
  // unwrap_or = set the default to zero in case in None
  let score =  scores.get(&team_name).copied().unwrap_or(0);
  println!("score: {score}\n");

  // loop through HashMap
  for (key, value) in &scores {
    println!("{key}: {value}");
  }
  println!("{:?}", scores);

  // HasMap Ownership
  let field_name = String::from("Favorite color");
  let field_value = 32;

  let mut mapper =  HashMap::new();
  mapper.insert(field_name, field_value);
  
  // println!("Field_name: {field_name}"); // out of score, owned by mapper HashMap
  println!("Field_name: {field_value}\n");  // stack data type, the value was only copied
  for(key, val) in &mapper{
    println!("{key}: {val}");
  }


  // Updating HashMap, Overwriting a value
  let mut scoring = HashMap::new();
  scoring.insert(String::from("Blue"), 10);
  scoring.insert(String::from("Blue"), 25);
  println!("{:?}", scoring);

  // Adding a Key and Value Only If a Key Isn’t Present
  // entry method is an enum called Entry that represents a value that might or might not exist
  scoring.entry(String::from("Yellow")).or_insert(50);
  scoring.entry(String::from("Blue")).or_insert(30);
  println!("{:?}\n", scoring);



  // Updating a Value Based on the Old Value
  let text = "hello world wonderful world";
  let mut text_map = HashMap::new();

  for word in text.split_whitespace(){
    let count = text_map.entry(word).or_insert(0); // or_insert return &mut V
    *count += 1;
  }
  println!("{:?}", text_map)

}
