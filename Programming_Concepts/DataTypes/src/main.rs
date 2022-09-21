fn main() {
  // Althougt the compiler infers the type when it is not specified.
  // In this case, We have to specify the data type as parse expect
  // a type to parse the string into.
  let guess: u32 = "42".parse().expect("Not a number");
  println!("The guess is: {guess}");

  // Scalar Types
  // => integers, floating-point numbers, boolean and characters

  // floating points
  let x = 2.0;  // f64
  let y: f32 = 3.2;  // f32

  // Numeric operations
  // addition
  let sum = 5 + 10;

  // substraction
  let difference = 95.2 - 4.2;

  // Mutliplication
  let product = 4 * 30;

  // division
  let division = 23.3 / 47.3;

  // remainder
  let remainder = 43 % 5;

}
