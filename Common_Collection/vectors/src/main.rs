fn main() {
  let mut v: Vec<i32> =  Vec::new();    // create an empty vector
  println!("{:?}", v);   // []

  v = vec![1,2,3,4,5];
  println!("{:?}", v);   // [1, 2, 3, 4, 5]

  v.push(6);
  v.push(7);
  println!("{:?}", v);

  // Reading Elements of Vectors
  let third = &v[2];
  println!("{third}");

  let third: Option<&i32> = v.get(2);  // get return an Option reference
  match third {
      Some(third) => println!("The third element is {}", third),
      None => println!("There is no third element"),
  }

  // Iterating over the Values in a Vector
  let k = vec![100, 32, 57];
  for i in &k {
      println!("{}", i);
  }
  // k.push(23); // will result to an error

  let mut s = vec![100, 32, 57];
  for i in &mut s {
      *i += 50;           // dereferencing, have access to the value of i and be able to modify it
  }
  s.push(10);


  let mut values = vec![1,2,3,4,5];
  let last = &values[0];
  
  // values.push(6);    // result to an error 

  println!("The last element is {last}");
  values.push(6);


}
