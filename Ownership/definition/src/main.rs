fn main() {
  // ----------------------------- Memory allocation -----------------------------

  // 1. ****** Stack-Only Data: Copy ******
      /*
          The copy is possible as these data-types are fixed value and their type are known during the compiler time.
          They are stored in on Stack during program execution.
      */

  // Integers
  let x = 5;
  println!("x before copying: {x}");
  let y = x;
  println!("x after copying: {x}, and y is: {y}");

  // float
  let f = 23.5;
  println!("f before copying: {f}");
  let f2 = f;
  println!("f after copying: {f}, and f2 is: {f2}");

  // character
  let c: char = 'c';
  println!("c before copying: {c}");
  let c2 = c;
  println!("c after copying: {c}, and c2 is: {c2}");

  // tuples
  let tup = (500, 6.4, 1);
  println!("tuple before copying: {:?}", tup);
  let tup2 =  tup;
  println!("tuple after copying: {:?}, and tup2 is: {:?}", tup, tup2);

  // array
  let a = [1,2,3,4,5];
  println!("array before copying: {:?}", a);
  let b = a;
  println!("array after copying: {:?}, and b is: {:?}", a, b);

  // string No-growable
  let s: &str = "Jacob Sima";
  println!("string before copying: {}", s);
  let s2 = s;
  println!("string after copying: {:?}, and s2 is: {:?}\n", s , s2);


  // 2. ****** Ways Variables and Data Interact: Move ******
    /*
      The data type is growable and stored in the Heap.
      it is pointing to the Heap, from stack with pointer(address) capacity and length.
      When we make a move or copy, we are actually making a copy of the representation
      in the stack
    */

  // String growable
  let str1 = String::from("Hello");
  println!("str1 from heap before copying: {}", str1);
  let str2 = str1;

  // This will result to an error as the value has been moved to str2
  // therefore str1 is out of scope.
  // println!("str1 from heap before copying: {}", str1); 
  println!("str1 moved to str2, str1 out of scope after copying, str2: {}", str2);

  // Vector growable
  let vec1:Vec<i32> = vec![1,2,3,4,5];
  println!("vec1 from heap before copying: {:?}", vec1);
  let vec2 = vec1;
  // println!("vec1 from heap before copying: {:?}", vec1);  // result to error, value moved to vec2
  println!("vec1 moved to vec2, vec1 out of scope after copying, vec2: {:?}\n", vec2);



  // 3. ****** Ways Variables and Data Interact: Clone ******
    /*
      In this case, We have create a copy of data in the Heap, which a very costly operation
      specially when the data is very large. str_1 and str_2 are both in scope and completly
      two different data with their own reference on the stack and data in the heap.
      changing one of them would not affect the other.
    */

  // String growable 
  let str_1 =  String::from("Mbote");
  let mut str_2 =  str_1.clone();
  println!("str_1 is: {}, str_2: {}, str_2 has cloned str_1", str_1, str_2);
  str_2.push_str(" Kuna");
  println!("str_1 is: {}, str_2: {}, mutate str_2 after str_2 cloned str_1", str_1, str_2);

  // Vector growable
  let mut vect_1 = vec!["hello", "mbote"];
  let vect_2 = vect_1.clone();
  println!("vect_1 is: {:?}, vect_2: {:?}, vect_2 has cloned str_1", vect_1, vect_2);
  vect_1.push("bondiya");
  vect_1.push("bonjour");
  println!("vect_1 is: {:?}, vect_2: {:?}, mutate vect_1 after vect_2 cloned vect_1\n", vect_1, vect_2);

  // ----------------------------- Ownership and Functions -----------------------------
    /*
      The mechanics of passing a value to a function are similar to those 
      when assigning a value to a variable
    */

  // Heap Data
  // s comes into scope and own the value of = string will be taken
  let s = String::from("string will be taken");  
  
  // s's value moves into the function and goes out of scope right here.
  takes_ownership(s);
  // println!("{s}");   // This will result to an error, as s is not own by takes_ownership function


  // Stack Data
  let x = 5;    // x comes into scope 
  make_copy(x);      // x would move into the function, but i32 is Copy, so it's okay to still
  println!("Even if x was copy but we can still print it here, x: {x}");


  // ----------------------------- Return Values and Scope -----------------------------
    /*
    Returning values can also transfer ownership
    */

    // given ownership
    let some_str_1 = gives_ownership(); // gives_ownership moves its return value into some_str_1
    println!("ownership was given to me from a function string is: {some_str_1}");


    // Give and take back ownership but with two different variables
    let some_str_2 = String::from("hello"); 
    let some_str_3 = takes_and_gives_back(some_str_2);
    println!("takes_and_gives_back : {some_str_3}"); 
    /*
      But this is too much ceremony and a lot of work for a concept that should be common. 
      Luckily for us, Rust has a feature for using a value without transferring ownership, 
      called references.
    */

    let some_a = String::from("hello");
    let (some_b, len) = calculate_length(some_a);
    println!("The length of '{}' is {}.", some_b, len);

}




fn takes_ownership(some_string: String){     // some_string comes into scope 

  println!("takes ownership own the string: {some_string}"); 

} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.


fn make_copy(some_integer: i32){  // some_integer comes into scope

  println!("x was copy here, x: {}", some_integer);

} // Here, some_integer goes out of scope. Nothing special happens.



fn gives_ownership() -> String { 
  let some_string = String::from("take it, it is yours");  // some_string comes into scope
  some_string
}  // some_string is returned and moves out to the calling function


// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
a_string  // a_string is returned and moves out to the calling function
}

// return a tuple
fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // len() returns the length of a String
  (s, length)
}
