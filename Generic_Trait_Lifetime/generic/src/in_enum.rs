pub fn run()
{
  println!("enum")
}

// standard library 
enum Option<T> 
{
  Some(T),
  None
}

// E = std::io::Error
enum Result<T, E> 
{
  Ok(T),
  Err(E)
}