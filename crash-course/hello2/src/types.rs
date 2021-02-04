
/*
primitive types:
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Floats: f32, f64
Boolean: bool
Character: char (unicode)
Tuples
Arrays

statically typed
compiler can usually infer type

var name convention is snake case
*/

pub fn run() {
  
  let x = 1;      // int - default i32
  let y = 2.5;    // float - default f64

  let z: f64 = 3.1415; // explicit type

  // println!("y: {}, z: {}", y, z);
  // warning: unused variable: `x`
  // help: if this is intentional, prefix it with an underscore: `_x`

  println!("x: {}, y: {}, z: {}", x, y, z);

  // find max size
  println!("Max i32: {}", std::i32::MAX); // 2147483647
  println!("Max i64: {}", std::i64::MAX); // 9223372036854775807

  // boolean
  let is_active = true;
  let is_explicit: bool = true;

  // bool from expression
  let is_greater = 10 > 5;
  let is_less_than: bool = 10 < 5;

  
  // char
  let a1 = 'a';
  // let face = '\u1f00'; // error: incorrect unicode escape sequence
    // help: format of unicode escape sequences uses braces: `\u{1f00}`
  let face = '\u{1f600}'; // '😀' - search for "emoji unicode"
  
  println!("{:?}", (x, y, z, is_active, is_explicit, is_greater, is_less_than, a1, face));
}