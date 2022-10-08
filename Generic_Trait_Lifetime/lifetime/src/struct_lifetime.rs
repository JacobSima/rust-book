// Like function, struct lifetime are only concerned for reference

struct DemStruct <'a>{
  data: Vec<i32>,
  reference: &'a Vec<i32>,
}


pub fn run(){
  println!("struct lifetime");
}