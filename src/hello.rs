#[allow(unused)]


// mod old;

// fn main() {
pub fn hello() { // error[E0601]: `main` function not found in crate `hello`
  // let x: i64 = 10;
  let x: i32 = 15;
  println!("Hello, World!");
  println!("Hello, fib({x}) == {}!", fib(x as i64));
  // std::fmt::
}

fn fib(n: i64) -> i64 {
  if n == 0 { return 0; }
  if n == 1 { return 1; }
  return fib(n - 1) + fib(n - 2);
}