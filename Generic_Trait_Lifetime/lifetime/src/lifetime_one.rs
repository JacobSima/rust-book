/*
  -> Rust allows one and only one owner of memory
  -> Rust allows multiple reference
  -> Lifetimes enforce a piece of memory is still valid for a reference
  Lifetime is for both Statck or Heap memory reference
*/

pub fn run(){

  let some_int_var = 20; 
  let some_int_var_2 = 10; 
  let result_ref =  get_ref(&some_int_var);  
  println!("result_ref: {result_ref}");

  let result_2 = some_func_3(&some_int_var, &some_int_var_2);
  println!("result_2: {result_2}");

  let vec_a: Vec<i32> = vec![1,2,3,4,5,6,7];
  let result = get_vec_slice(&vec_a);
  println!("{:?}", result);

  let result_2 = get_vec_slice_2(&vec_a, &vec_a);
  println!("{:?}", result_2);

}


fn scope_owner(){
  let a;
  {
    let b = String::from("Howdy");

    // dangling reference as b will be out of scope, a will reference some garbage
    // which is not allowed in Rust
    // a = &b;  
    a = b;
  } // b goes out of scope, and a takes ownership of b
  println!("a: {a}");
}


/*

// Not reference was passed as parameter, then we can't return one unless is a static reference
fn get_int_ref() -> &i32 {
  let a = 1;
  &a
  // a out of scope, memory gets clean-up the life of the memory to return is cleaned-up
}

*/

// valid, we are given ownership to the calling function.
fn get_int_ref() -> i32 {
  let a = 1;
  a
}

// valid as the function took a reference and return the same exact reference
// 'a : generic lifetime automatically added by Rust under the wood
// the input and output memory live at the same scope and lifetime 
fn get_ref<'a>(param_1: &'a i32) -> &'a i32 {   // this was verbose
  param_1
}
fn get_ref_1(param_1: &i32) -> &i32 {   // lifetime automated by Rust
  param_1
}


// 'a, 'b = lifetime references of param_1 and param_2
// param_3 does not used reference, therefore there is not generic lifetime reference
// return a String with not reference, then No problem here with lifetime
fn some_func<'a, 'b>(param_1: &'a str, param_2: &'b str, param_3: Vec<i64>) -> String{
  String::from("string to return")
}


// return reference of param_1 lifetime type
// param_2 does not use reference, then there is no issue
fn some_func_2<'a>(param_1: &'a i32, param_2: i32) -> &'a i32 {param_1}


// Facing lifetime issue, as there is not garantee that a or b will both live longer
// with equal lifetime
// fn some_func_3<'a, 'b>(param_1: &'a i32, param_2: &'b i32)
// 'b: 'a = b lives as long as as a (lifetime sub-typing)
// a and b have the same lifetime in the scope
fn some_func_3<'a>(param_1: &'a i32, param_2: &'a i32) -> &'a i32 { // now lifetime for both is garanty to be the same
  if param_1 < param_2 {
    param_1
  }else{
    param_2
  }
}

fn get_str<'a>(param_1: &'a str, param_2: &'a str) -> &'a str {
  if param_1 > param_2 {
    param_1
  }else{
    param_2
  }
}

// Do not apply lifetime
// No lifetime required here, as we not returning a reference and there is not reference parameter
fn test_1(param_1: Vec<f64>) -> Vec<f64>{
  param_1
}

// No lifetime applied as the return is givin ownership, to reference return used
fn test_2<'a>(param_1: &'a Vec<f64>) -> Vec<f64> {
  param_1.clone()
}


fn test_3<'a>(param_1: i32, param_2: &'a str, param_3: &'a str, param_4: f64) -> &'a str {
  if param_1 == 7 && param_4 > 10. {
    param_2
  }else{
    param_3
  }
}

// Valid
// If a vector has a lcertain lifetime, all the slice element have the same lifetime
fn get_vec_slice(param_1: &[i32]) -> &[i32] {
  &param_1[0..2]
}

fn get_vec_slice_2<'a>(param_1: &'a [i32], param_2: &'a [i32]) -> &'a [i32] {
  if param_1.len() > param_2.len() {
    &param_1[0..2]
  }else{
    &param_2[2..]
  }
}


/*
Static lifetime  means a lifetime that lats the entire program
Constant are static by their nature
can also have static variables

*/
const SOME_CONST_A: &str = "There is one";
const SOME_CONST_B: &str = "Line Two";
const SOME_CONST_C: &str = "Third line";
fn some_func_static() -> &'static str {
  SOME_CONST_A
}

fn some_func_static_2(param_1: &'static str, param_2: &'static str) -> &'static str {
  if param_1 > param_2 {
    param_1
  }else{
    param_2
  }
} 