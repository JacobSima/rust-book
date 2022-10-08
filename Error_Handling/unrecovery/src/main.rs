#![allow(unused)]
/*
Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond 
the end of an array, and so we want to immediately stop the program

Memory that the program was using will then need to be cleaned up by the operating system. 
If in your project you need to make the resulting binary as small as possible, 
you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to 
the appropriate [profile] sections in your Cargo.toml file. For example, 
if you want to abort on panic in release mode, add this:

[profile.release]
panic = 'abort'
*/

fn main() {
  // panic!("crash and burn");

  let v = vec![1, 2, 3];
  v[99];
}
