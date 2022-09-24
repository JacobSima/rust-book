struct Rectangle {
  width: u32,
  height: u32
}

pub fn run(){
  let rect =  Rectangle{
    width: 30,
    height: 50,
  };
  println!("\nThe area of the rectangle is {} square pixels", area(&rect));
  // println!("The Rectangle is: {:#?}", rect);

}

fn area(rect: &Rectangle) -> u32 {
  rect.width * rect.height
}