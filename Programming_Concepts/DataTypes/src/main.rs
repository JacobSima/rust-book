use std::io::{stdin};

fn main() {
  // Althougt the compiler infers the type when it is not specified.
  // In this case, We have to specify the data type as parse expect
  // a type to parse the string into.
  let guess: u32 = "42".parse().expect("Not a number");
  println!("The guess is: {guess}");

  // All Scalar and Compound Types are stored in the Stack during the program execution.
  // The type and values are knwon at the compiler time.

  // Scalar Types
  // => integers, floating-point numbers, boolean and characters

  // floating points
  let x = 2.0;  // f64
  let y: f32 = 3.2;  // f32

  // Numeric operations
  // addition
  let sum = 5 + 10;

  // substraction
  let difference = y - x;

  // Mutliplication
  let product = 4 * sum;

  // division
  let division = 23.3 / difference;

  // remainder
  let remainder = product % 5;

  println!("sum: {sum}, difference: {difference}, product: {product}, division: {division}");
  println!("remainder: {remainder}");

  // Boolean
  let t = true;
  let f: bool = false;    // with explicit type annotation

  println!("t: {t}, f: {f}");

  // Character => 4 bytes in size
  let c = 'z';
  let z: char = 'Z';    // with explicit type annotation
  let heart_eyed_cat = '😻';   // Emoji

  println!("Smal z: {c}, Capital Z: {z}, Emoji: {heart_eyed_cat}");

  // Compoud Types

  // Tuple
  // Use pattern matching to destructure a tuple value
  let tup: (i32, f64, u8) = (500, 6.4, 1); 

  // Also applied shadowing on x and y, sice they were defined on top, we let them again
  let (x, y,z) = tup;    
  println!(" The value of x, y and z are respectively: {x}, {y}, {z}");
  println!(" The value of x, y and z are respectively: {}, {}, {}", tup.0, tup.1, tup.2);

  // Array fixed length
  // However, arrays are more useful when you know the number of elements will not need to change
  let a = [1,2,3,4,5];
  let b: [i32; 5] = [6, 7, 8, 9, 10];
  let c = [3; 5];    // [3, 3, 3, 3, 3]
  let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

  let first = a[0];
  let second = b[1];
  let third = c[2];
  let birth_month = months[7];

  println!("First: {first}, Second: {second}, Third: {third}, Birth month: {birth_month}");


  // Small program to simulate Invalid Array Element Access
  println!("Months of the year are: {:?}", months);
  println!("Please enter a number to fetch the month: ");

  let mut user_input = String::new();
  stdin().read_line(&mut user_input).expect("Failed to get the number");
  
  // Let take care of the exception
  // let user_input: usize = user_input.trim().parse().expect("Please Enter a number");
  let mut user_input: usize = match user_input.trim().parse(){
    Ok(num) => num,
    Err(_) => {
      println!("Please enter a number between 0 - 11");
      println!("Must be between Number, therefore, you will get the January");
      0
    }
  };

  user_input = match user_input{
    x @ 0..=11 => x ,
    _ => 0,
  };
  println!("The month is: {}", months[user_input]);

}
