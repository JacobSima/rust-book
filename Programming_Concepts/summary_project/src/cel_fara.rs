pub fn celcius_to_farenheit(a: [i32; 5]) -> [f64; 5] {
  let mut i = 0;
  let mut b:[f64; 5] = [1.1; 5];
  while i < 5{
    b[i] = (a[i] as f64 * (1.7)) + 32 as f64 ;
    i += 1;
  }
  b
}

pub fn for_loop_mutate(mut a: [i32; 5]) -> [i32; 5]{
  for num in a.iter_mut(){
    *num = ((*num as f64 * (1.7)) + 32 as f64 ) as i32;
  }
  a
}