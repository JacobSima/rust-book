use core::panic;
use std::fmt::format;

/*
The default behavior of the binary produced by cargo test is to run all the tests in parallel 
and capture output generated during test runs, preventing the output from being displayed 
and making it easier to read the output related to the test results

cargo test --help:  displays the options you can use with cargo test,
cargo test -- --help:  displays the options you can use after the separator.
cargo test -- --test-threads=1: test takes longer, If you don’t want to run the tests in parallel
the tests won’t interfere with each other if they share state.
cargo test -- --show-output:  If we want to see printed values for passing tests 
cargo test name_of_test_function: Running Single Test
cargo test hold -:  run test function that contain the n word 'hold'(Filtering to Run Multiple Tests)
cargo test -- --ignored : run only ingored test functions
*/

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
  format!("Hello {name}!")
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

pub struct Guess {
  value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
      if value < 1 {
        panic!("Guess value must be greater than or equal to 1, got {}", value);
      } else if value > 100 {
        panic!("Guess value must be less than or equal to 100, got {}", value);
      }
      Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This attribute indicates this is a test function
    // cargo test: runs all test in our project
    #[test]   
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration(){
      assert_eq!(2+2, 4);
    }

    // Create a failed test
    // #[test]
    // fn another(){
    //   panic!("Make this test fail");
    // }


    #[test]
    fn larger_can_hold_smaller(){
      let larger = Rectangle{
        width: 8,
        height: 7,
      };

      let smaller = Rectangle{
        width: 5,
        height: 1,
      };

      assert!(larger.can_hold(&smaller));
    }


    #[test]
    fn smaller_cannot_hold_larger(){
      let larger = Rectangle{
        width: 8,
        height: 7,
      };

      let smaller = Rectangle {
        width: 5,
        height: 1,
      };

      assert!(!smaller.can_hold(&larger));
    }


    // Adding Custom Failure Messages
    #[test]
    fn greeting_contains_name(){
      let result = greeting("Carol");
      // assert with custom error message
      assert!(result.contains("Carol"), "Greeting did not contain name, value was '{}'", result);  
    }


    // Checking for Panics with should_panic
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100(){
      Guess::new(200);
    }


    // Using Result<T, E> in Tests
    #[test]
    #[ignore = "test logic only"]
    fn test_work() -> Result<(), String> {
      if 2 + 2 == 4{
        Ok(())
      }else {
        Err(String::from("two plus two does not equal four"))
      }
    }

}
