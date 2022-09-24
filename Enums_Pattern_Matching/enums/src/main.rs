#![allow(unused)]


// We attach data to each variant of the enum directly, so there is no need for an extra struct
enum IpAddr {
  V4(String),  // V4(u8, u8, u8, u8),
  V6(String),
}

struct Ipv4Addr {}
struct Ipv6Addr {}

enum IpAddress {   // enum can hold even struct data type
  V4(Ipv4Addr),
  V6(Ipv6Addr),
}

enum Message {
  Quit,
  Move {x: i32, y: i32},
  Write(String),
  ChangeColor(i32, i32, i32),
}
// enum can also have method syntax
impl Message {
  fn call(&self) { println!("\nMessage enum called\n")}
}

// We also define struct based simular to this enum, but in this case the enum 
// it is the most concise way 
struct QuitMessage; // unit struct
struct MoveMessage {x: i32,y: i32,}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

struct MessageStruct {
  quit: QuitMessage,
  moves: MoveMessage,
  writes: WriteMessage,
  change_color: ChangeColorMessage,
}


/*
   The Option Enum and Its Advantages Over Null Values
   Option is another enum defined by the standard library; Option =  there something or nothing
    enum Option<T> {
      None,
      Some(T),
    }
*/

fn main() {

  let home = IpAddr::V4(String::from("127.0.0.1"));
  let loopback = IpAddr::V6(String::from("::1"));
  let message_move = Message::Move { x:12, y: 32 };
  message_move.call();

  let some_number = Some(5);        // Option<i32>
  let some_char = Some('e');      // Option<char>

  // If None then can't infer type, you need to explicitly set the type
  let absent_number: Option<i32> = None;
  println!("absent_number: {}", absent_number.unwrap_or(1));   // default set to 0

  let x: i8 = 5;
  let y: Option<i8> = Some(5);
  println!("y: {}", y.unwrap_or(0));     // got 5 as number

  let sum = x + y.unwrap_or(0); // 5 + 5
  println!("Sum is: {}", sum);

  // Assert 
  assert_eq!(Some("car").unwrap_or("bike"), "car");
  assert_eq!(None.unwrap_or("bike"), "bike");
}


