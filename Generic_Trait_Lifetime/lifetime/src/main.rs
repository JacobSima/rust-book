#![allow(unused)]
mod lifetime_one;
mod lifetime_with_generic;
mod struct_lifetime;

// &i32                     // a reference
// &'a i32                  // a reference with an explicit lifetime
// &'a mut i32              // a mutable reference with an explicit lifetime

// lifetimes ensure that references are valid as long as we need them to be
// every reference in Rust has a lifetime, which is the scope for which that reference is valid
fn main() {
  // Lifetime Example one
  lifetime_one::run();

  // Lifetime with Generic
  lifetime_with_generic::run();

  // Struct lifetime
  struct_lifetime::run();
}
