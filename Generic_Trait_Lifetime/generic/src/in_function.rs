use std::cmp::PartialOrd;

pub fn run()
{
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest_gen(&number_list);
  println!("The largest number is {result}");

  let char_list = vec!['y', 'm', 'a', 'q'];
  let result = largest_gen(&char_list);
  println!("The largest char is {result}\n");
}

fn largest_i32(list: &[i32]) -> &i32 {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest =  item;
    }
  }
  largest
}

fn largest_char(list: &[char]) -> &char {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest =  item;
    }
  }
  largest
}

// Generic
// fn largest_gen<T: PartialOrd>(list: &[T]) -> &T
fn largest_gen<T>(list: &[T]) -> &T 
where T: PartialOrd
{
  let mut largest = &list[0];

  for item in list{
    if item > largest {
      largest = item
    }
  }
  largest
}