use adder;

pub fn setup_common() {
  let result = adder::greeting("Jacob");
  assert!(result.contains("Jacob"));
}