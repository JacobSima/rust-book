
// Calling Function
fn main() {
    println!("Hello, world!");
    another_function();
    with_params(5);
    statement_expression();

    // FUnction returning a value
    println!("The value from function return is: {}", function_return_value());
    let result = plus_one(function_return_value());
    println!("The result of function_return_value plus one is: {result}");
}

// Called Function
// Called function can be define before or after the Calling function
fn another_function(){
  println!("Another function...");
}

fn with_params(x: i32){
  println!("The value of x is: {x}");
  print_labeled_measurement(10, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char){
  println!("The measurement is: {value}{unit_label}");
}

fn statement_expression(){
  // Rust is an expression-based language,
  // Statements and Expressions
  // Statements are instructions that perform some action and do not return a value
  // Expressions evaluate to a resulting value

  // let x = (let y = 6);   // Wrong in Rust as (let y = 6) does not return a value
  // let x = y = 6     // not allow in Rust

  // Everything inside the {} is an expression. As it returns a value
  let y = {
    let x = 3;
    // Expressions do not include ending semicolons
    // If you add a semicolon to the end of an expression, you turn it into a statemen
    x + 1     
  };

  println!("The value of y is: {y}");
}

fn function_return_value() -> i32 {5}

fn plus_one(x: i32) -> i32 { x + 1}

