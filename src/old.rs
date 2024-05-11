
// [Rust for Old People. How life after C, C++, and Python feels…  by Doug Foo  CodeX  Medium ](https://medium.com/codex/rust-for-old-people-516fc72b2934)

use std::time::SystemTime;

pub fn old() {
  let x: i32 = 15;
  println!("Rust for Old People!");
  let _start = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
  println!("Hello, fib({x}) == {}!", fib(x as i64));


}

// non-Tail Call Optimised
fn fib(n: i64) -> i64 {
  if n == 0 { return 0; }
  if n == 1 { return 1; }
  return fib(n - 1) + fib(n - 2);
}

// Tail Call Optimised
fn fib2(n: i64) -> i64 {
  fn fib2_inner(a: i64, b: i64, n: i64) -> i64 {
    if n == 0 { return 0; }
    return fib2_inner(b, a+b, n-1);
  }
  return fib2_inner(0, 1, n);
}