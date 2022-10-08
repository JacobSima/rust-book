pub fn run(){
  let some_float_1 = 1.1;
  let some_float_2 = 1.5;
  
  let result =  get_smaller(&some_float_1, &some_float_2);
  println!("\nresult: {result}");

  let some_str_1 = "bana mayi";
  let some_str_2 = "Bana mayi ba boyi na bango";

  let result =  get_smaller(&some_str_1, &some_str_2);
  println!("result: {result}");



}

fn get_smaller<'a, T>(param_1: &'a T, param_2: &'a T) -> &'a T 
where T: std::cmp::PartialOrd {
  if param_1 > param_2 {
   return param_1;
  }
  param_2
}