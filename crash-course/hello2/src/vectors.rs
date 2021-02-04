
// vectors - resizable arrays

use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

  println!("memory: {} bytes", mem::size_of_val(&numbers)); // memory: 24 bytes
  println!("{:?}", numbers);
  
  // re-assign
  numbers[2] = 20;
  println!("{:?}", numbers);
  
  println!("Single value: {}", numbers[0]);
  
  println!("Vector length: {}", numbers.len());
  
  numbers.push(5);
  numbers.push(6);
  
  println!("{:?}", numbers);
  println!("memory: {} bytes", mem::size_of_val(&numbers)); // memory: 24 bytes
  
  numbers.push(7);
  println!("{:?}", numbers);
  println!("memory: {} bytes", mem::size_of_val(&numbers)); // memory: 24 bytes ??

  numbers.push(8);
  println!("{:?}", numbers);
  println!("memory: {} bytes", mem::size_of_val(&numbers)); // memory: 24 bytes ??

  numbers.pop();

  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  for x in numbers.iter_mut() {
    *x *= 2;
  }
  println!("{:?}", numbers); // needs debug formatting trait
}