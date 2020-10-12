
// primitive str - immutable, fixed length, in mem (string literal?)
// String - growable, heap-allocated

pub fn run() {
  let hello = String::from("Hello");
  let literal = "literal";

  println!("{}", hello);
  println!("Length of hello: String: {}", hello.len()); // 5
  println!("Length: literal: str: {}", literal.len());  // 7 - .len() works on string literals

  let mut mut_hello = String::from("Hello ");
  
  mut_hello.push('W'); // push single char
  println!("{}", mut_hello); // Hello W
  
  mut_hello.push_str("orld!");  // push str
  println!("{}", mut_hello);    // Hello World!

  // capacity in bytes
  println!("Capacity: {}", mut_hello.capacity());
  
  
}