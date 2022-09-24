fn main() {
  /*
    Slices let you reference a contiguous sequence of elements in a collection rather 
    than the whole collection.A slice is a kind of reference, so it does not have ownership.
  */

  // String Slices
  let s = String::from("hello world");
  let hello =  &s[0..5];
  let world = &s[6..11];
  println!("&s[0..5]: {hello}, &s[6..11]: {world}\n");
  
  let len = s.len();
  let slice_1 = &s[3..len];
  let slice_2 = &s[3..];
  println!("&s[3..len]: {slice_1}, &s[3..]: {slice_2}\n");

  let slice_3 = &s[0..len];
  let slice_4 = &s[..];
  println!("&s[0..len]: {slice_3}, &s[..]: {slice_4}\n");

  // Array slice
  let a = [1, 2, 3, 4, 5];
  let b = &a[1..3];
  assert_eq!(b, &[2, 3]);
  println!("a: {:?}, &a[1..3]: {:?}", a, b);
}


// fn first_word(s: &String) -> &str {
//   let bytes = s.as_bytes();

//   for (i, &item) in bytes.iter().enumerate() {
//       if item == b' ' {
//           return &s[0..i];
//       }
//   }

//   &s[..]
// }