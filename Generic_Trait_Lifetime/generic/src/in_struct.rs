pub fn run()
{
  let integer = Point{x:5, y:10};
  let float = Point{x:1.5, y:4.0};

  println!("integer: {:#?}", integer);
  println!("float: {:#?}", float);
  println!("float.x = {}\n", float.x());

  let rectangle = Rectangle{x:10, y:3.2};
  println!("{:#?}", rectangle);
  
}

#[derive(Debug)]
struct Point<T> 
{
  x : T,
  y : T
}

impl <T> Point<T> {
  fn x(&self) -> &T 
  {
    &self.x
  }
}

#[derive(Debug)]
struct Rectangle<T, U>
{
  x : T,
  y : U
}

