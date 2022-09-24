#![allow(unused)]
#[derive(Debug)] 
enum UsState {
  Alabama,
  Alaska,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn main() {
  
  // let c = Coin::Penny;
  let c = Coin::Quarter(UsState::Alabama);
  let coin = value_in_cents(c);
  println!("Coin is: {}\n", coin);  

  // Matching with Option<T>
  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
  println!("six: {:?}, none: {:?}", six, none);

  // Catch-all Patterns and the _ Placeholder
  let dice_roll = 9;
  match dice_roll {
      3 => add_fancy_hat(),
      7 => remove_fancy_hat(),
      // _ => reroll(),  // call reroll()  it is like default in switch case
      _ => ()  // do nothing
  }

  
  // Concise Control Flow with if let
  // let config_max = Some(3u8);
  // match config_max {
  //     Some(max) => println!("The maximum is configured to be {}", max),
  //     _ => (),
  // }

  let config_max = Some(3u8);
  if let Some(max) = config_max {
      println!("The maximum is configured to be {}", max);
  }


}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
      println!("Lucky number");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    },
  }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
      None => None,
      Some(i) => Some(i + 1),
  }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
