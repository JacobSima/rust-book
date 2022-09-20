fn do_something() {
  println!("Hello there");
  let x = std::f64::consts::PI;
  let r = 8.0;
  println!("the area of the circle is {}", x * r * r);
}

fn main() {
  for _i in 0..10{
    do_something();
  }
}