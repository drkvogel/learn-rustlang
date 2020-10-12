// vars hold primitive data types or references
// immutable by default
// block-scoped


pub fn run() {
  let name = "Chris";

  // let age = 45;
  // age = 46; // cannot assign twice to immutable variable

  let mut age = 45; // warning: value assigned to `age` is never read
  // = note: `#[warn(unused_assignments)]` on by default
  // = help: maybe it is overwritten before being read?
  println!("My name is {} and I am {}", name, age); // My name is Chris and I am 45
  age = 46;
  println!("My name is {} and I am {}", name, age); // My name is Chris and I am 46

  // constant
  // const id: i32 = 001; // warning: constant `id` should have an upper case name
  const ID: i32 = 001; 

  println!("id: {}", ID);

  let (my_name, my_age) = ("Chris", 45);
  println!("{} is {}", my_name, my_age); // Chris is 45

}