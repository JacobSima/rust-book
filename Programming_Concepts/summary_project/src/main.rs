mod cel_fara;
mod fibonacci;
mod repetiion_string;

fn main() {
  let a = [1,2,3,4,5];
  println!("The Array in Celius is: {:?}", a);
  let a = cel_fara::celcius_to_farenheit(a);
  println!("The Array in Farenheit is: {:?}", a);

  let mut b = [6,7,8,9,10];
  println!("The Array in Celius is: {:?}", b);
  b = cel_fara::for_loop_mutate(b);
  println!("The Array in Farenheit is: {:?}\n", b);

  fibonacci::run();

  repetiion_string::run();
}
