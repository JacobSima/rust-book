fn main() {

  // 1. ----------- References and Borrowing -----------

  /*
    A reference is like a pointer in that it’s an address we can follow to access 
    the data stored at that address; that data is owned by some other variable. 
    Unlike a pointer, a reference is guaranteed to point to a valid value of a particular 
    type for the life of that reference.
  */

  // s1 pointing to the Heap (prt, len, capacity)
  let s1 = String::from("hello");  
  
  // len reference s1(ptr), does not take ownership of s1.
  let len = calculate_length(&s1);

  // s1 and len both are still in the scope.
  // len has access to s1 value.
  println!("The length of {s1} is {len}.\n");


  // 2. ----------- Mutable References -----------
  let mut some_str = String::from("Boni ");
  println!("some_str before change: {some_str}");   
  change_str(&mut some_str);  // borrow some_str reference
  println!("some_str after change: {some_str}\n");    // some_str is still in the scope


  // 3. ----------- Mutable References Restriction -----------
  // If you have a mutable reference to a value, you can have no other references to that value 
  
  let mut greet = String::from("Hello");
  let r1 =  &mut greet ;
  // let r2 =  &mut greet ;           // Error only one mutable is allowed in a single scope

  // println!("{}, {}", r1, r2);      // Error r2 is not defined here.

  // Error, greet was borrowed as mutate but never being mutated, greet can be used until
  // it is mutate by r1 first and r1 has been used.
  // println!("greet is: {}\n", greet);    

  r1.push_str(" Lunda");
  // println!("greet is: {}\n", greet);   // Error r1 mutated but not used yet.
  println!("r1 is: {}", r1);
  // r1 is out of scope here
  println!("greet is: {}\n", greet);  // r1 was mutate and used, then we can use greet
  
  // declare another mutable as r1 is out of scope
  let r2 = &mut greet;   
  println!("r2 is: {}", r2);

  //********* block scope mutability *********
  let mut name = String::from("Jacob");
  println!("name before block scope: {name}");
  {
    let first_names = &mut name;
    first_names.push_str(" Kwetila");
  } // first_names go out of scope therefore we can define another mutable variable below
  println!("name after block scope: {name}"); // name can be used as first_names is out of scope

  let full_name = &mut name;
  full_name.push_str(" Sima");

  // println!("name is: {}", name);       // name cannot be used as full_name is still in the scope
  println!("full_name is: {full_name}");  // full_name used and out of scope
  println!("name is: {}\n", name); // name can be used as full_name used and out of scope

  //********* Immutability and mutability mixed *********
  let mut str_1 = String::from("hello");

  let p1 = &str_1; // no problem
  let p2 = &str_1; // no problem
  // let p3 = &mut str_1; // BIG PROBLEM as p1 and p2 is still in the scope
  println!("p1: {} and p2: {}", p1, p2);  // p1 and p2 are out of scope now.

  let p3 = &mut str_1; // this becomes valid
  println!("p3: {}", p3);


  // 4. ----------- Dangling References -----------
  // Dangling occurs when the pointer of a data is out of scope but its reference is still in scope
  // Meaning the reference will reference a no-existing pointer.

  /*
    let reference_to_nothing = dangle();
    fn dangle() -> &String {   // dangle returns a reference to a String
    l et s = String::from("hello");
      &s
    } // return a reference to String s , which is out of scope already. Danger
    That means this reference would be pointing to an invalid String. 
    That’s no good! Rust won’t let us do this.
  */

  fn no_dangle() -> String {
    let s = String::from("hello");
    s // pass onwership out of this function.
  } // This works without any problems. Ownership is moved out, and nothing is deallocated.
  


  /*  The Rules of References
    * At any given time, you can have either one mutable reference or any number of immutable references.
    * References must always be valid.
  */


}


fn calculate_length(s: &String) -> usize { // s is a reference to a String

  // This will result to an error as the s1 is not mutable and the reference in this
  // function is not mutable too
  // s.push_str("there");  

  s.len() 

} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, it is not dropped.


// s: &mut String, change_str function will mutate the string value it borrows.
fn change_str(s: &mut String){  
  s.push_str("kuna na Kikwit");
}








