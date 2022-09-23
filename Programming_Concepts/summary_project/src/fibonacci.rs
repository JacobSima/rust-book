// The Rule is Xn = (Xn-1) + (Xn-2)
// If n = 0   => 0
// If n = 1   => 1
//n =	  | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8  | 9  | 10 | 11 | 12  | 13  | 14  |
//xn =  | 0 | 1 | 1 | 2 | 3 | 5 | 8 | 13| 21 | 34 | 55 | 89 | 144 | 233 | 377 |

pub fn run(){
  let mut fibo_arr: Vec<i32> = vec![];
  for i in 0..=20 {
    let x = fibonacci(i);
    fibo_arr.push(x);
  }
  println!("\nThe value of x is: {:?}\n", fibo_arr);
}

pub fn fibonacci(x: i32) -> i32{
  if x == 0 {
    return 0;
  }
  if x == 1 {
    return 1
  }
  return fibonacci(x - 1) + fibonacci(x - 2);
}

