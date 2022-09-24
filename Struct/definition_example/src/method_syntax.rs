struct Rectangle {
  width: u32,
  height: u32
}

// The &self is actually short for self: &Self. Within an impl block
// the type Self is an alias for the type that the impl block is for
// &mut self  will be used if we want to mutate the value
// &self : we don’t want to take ownership
impl Rectangle {
    fn area(&self) -> u32 {
      self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
      self.width > other.width && self.height > other.height
  }
}

pub fn run(){
  let rect1 =  Rectangle{
    width: 10,
    height: 5,
  };

  let rect2 =  Rectangle{
    width: 50,
    height: 90,
  };


  // println!("\nThe area of the rectangle is {} square pixels", area(&rect));
  println!("\nThe area of the rectangle is {} square pixels", rect1.area());
  println!("can rect1 hold rect2: {}", rect1.can_hold(&rect2));
  println!("can rect2 hold rect1: {}", rect2.can_hold(&rect1));

}

// fn area(rect: &Rectangle) -> u32 {
//   rect.width * rect.height
// }