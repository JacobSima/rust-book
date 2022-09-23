fn main() {
  if_statement();
  else_if_statement();
  turnary_operator();

  // loop
  repetition_with_loop();
  multiple_loop();

  // Conditional loop with While
  while_loop();

  // Forloop 
  // While loop is a bit error prone, the better alternative is the forloop
  for_loop();

  // Forloop wit mutation in iteration
  let mut x = [1,3,5];
  x = for_loop_iter_mut(x);
  assert_eq!(x, [3, 5, 7]);    // Test the result

}

fn if_statement(){
  let number = 3;

  // You must be explicit and always provide if with a Boolean as its condition
  if number < 1 {
    println!("Condition was true");
  }else{
    println!("Condition was false");
  }
}

fn else_if_statement(){
  let number = 1;

  if number % 4 == 0 {
    println!("number is divisible by 4");
  }else if number % 3 == 0 {
    println!("number is divisible by 3");
  }else if number % 2 == 0 {
      println!("number is divisible by 2");
  }else{
    println!("number is not divisible by 4,3,2");
  }
}

fn turnary_operator(){
  let condition = false;
  let number =  if condition { 5 } else { 6 };
  println!("The ternary operator result is : {number}");
}

fn repetition_with_loop(){
  // The loop keyword tells Rust to execute a block of code over and over again forever 
  // or until you explicitly tell it to stop.
  // break: is used to break out from the loop
  // continue is used to skip everything below the continue statement and go to the next iteration
  let mut counter = 0;
  let result = loop {
    counter += 1;
    
    if counter == 10{
      break counter * 2;
    }

    if counter % 3 != 0 { continue;}
    println!("The counter has reached a number divisible by 3 and the counter is: {counter}");
  };
  println!("The loop counter result is is {result}");
}

fn multiple_loop(){
  println!("\n");
  let mut count = 0;

  'counter_first: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remianing = {remaining}\n");

      if remaining == 8 { break;}

      if count == 2 {
        break 'counter_first;
      }
      remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {count}\n");
}

// This construct eliminates a lot of nesting that would be necessary if 
// you used loop, if, else, and break, and it’s clearer
fn while_loop(){
  let mut number = 3;

  while number != 0{
    println!("{number}");
    number -= 1;
  }
  println!("LIFTOFF!!!\n");

  println!("Looping through a collection");
  let a = [10, 20, 30, 40, 50];
  let mut index = 0;

  while index < a.len() {
    println!("The value is: {}", a[index]);
    index += 1;
  }
}


fn for_loop(){
  println!("\n");

  let fruits = ["Apples", "Mangos", "Tomatoes", "Oranges", "grapes"];
  for fruit in fruits {
    println!("Fruit in the collection is: {fruit}");
  }
  
  println!("\n");
  for number in (1..=4).rev(){
    println!("{number}!");
  }

  println!("LIFTOFF!!!");
}


// fn for_loop_iter(){

// }


fn for_loop_iter_mut(mut x: [i32; 3]) -> [i32; 3]{
  for elem in x.iter_mut() {
      *elem += 2;
  }
  x
}



