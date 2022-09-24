pub fn run(){
  let width: u32 = 30;
  let height: u32 = 50;

  println!("\nThe area of the rectangle is {} square pixels", area(width, height))
}

fn area(a: u32, b: u32) -> u32 {
  a * b
}