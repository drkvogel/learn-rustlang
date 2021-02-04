
// arrays - fixed list where elements are the same data type

use std::mem; // https://youtu.be/zF34dRivLOw?t=3187 "Always a good thing to get rid of STDs"... hoho

pub fn run() {
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

  numbers[1] = numbers[3];

  println!("{:?}", numbers);                          // [1, 4, 3, 4, 5]
  
  println!("single value: {}", numbers[0]);           // single value: 1

  println!("array length: {}", numbers.len());        // array length: 5

  // println!("memory: {} bytes", std::mem::size_of_val(&numbers)); // memory: 20 bytes
  println!("memory: {} bytes", mem::size_of_val(&numbers)); // memory: 20 bytes

  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);
}