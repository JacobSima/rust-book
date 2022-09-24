struct Rectangle {
  width: u32,
  height: u32
}


impl Rectangle {
    fn area(&self) -> u32 {
      self.width * self.height
    }

    // Use as a constructor
    // let sq = Rectangle::square(3)
    fn square(size: u32) -> Self {
      Self {
        width: size,
        height: size,
      }
    }
}

// Multiple impl block is a valid Struct implementation
// Mostly used in generic types and traits.
impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
      self.width > other.width && self.height > other.height
  }
}

pub fn run(){
  let rect1 =  Rectangle{
    width: 10,
    height: 5,
  };

  let square = Rectangle::square(10);

  println!("\nThe area of the rectangle is {} square pixels", rect1.area());
  println!("\nThe area of the square is {} square pixels", square.area());

}

