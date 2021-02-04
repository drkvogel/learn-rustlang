
pub fn run() {
  greeting("Hello", "Dave");

  let sum = add(1, 2);
  println!("Sum: {}", sum);
  println!("Sum: {}", add(1, 2));

  // closure
  // let mut n3: i32 = 10; // warning: variable does not need to be mutable. help: remove this `mut`
  let n3: i32 = 10;
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; // closure can access "outside variables" from enclosing fn scope, unlike fn which is block-scoped
  println!("C sum: {}", add_nums(3, 3)); // 16
  
  // n3 = 12; // cannot assign to `n3` because it is borrowed

  println!("C sum: {}", add_nums(2, 2)); // 14

  // println!(add_print(1, 2)); // format argument must be a string literal
  // println!("{}", add_print(1, 2));
  // println!(add_print(1, 2));
}

fn greeting(greet: &str, name: &str) {
  println!("{} {}, nice to meet you", greet, name);
}

// fn add_print(n1: i32, n2: i32) -> &str { // expected named lifetime parameter
//   String(std::fmt("{} + {} = {}", n1, n2, add(n1, n2)));
// }
//  = help: this function's return type contains a borrowed value with an elided lifetime, but the lifetime cannot be derived from the arguments
// help: consider using the `'static` lifetime

// fn add_print(n1: i32, n2: i32) -> &'static str { // expected named lifetime parameter
// fn add_print(n1: i32, n2: i32) -> dyn String { // expected named lifetime parameter
//   format!("{} + {} = {}", n1, n2, add(n1, n2))
// }

fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2  // lack of semicolon implies return value
}