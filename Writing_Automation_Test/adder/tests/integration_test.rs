use::adder;
mod common;

/*
We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)]. 
Cargo treats the tests directory specially and compiles files in this directory only 
when we run cargo test

cargo test: run the entire test of the application
cargo test --test integration_test : run this integration test file only
*/

#[test]
fn workers(){
  assert_eq!(4, adder::add(2, 2));
}

#[test]
fn common_setup_common(){
  common::setup_common();
}