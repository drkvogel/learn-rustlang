
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
  println!("Capacity: {}", mut_hello.capacity()); // 12
  
  println!("Is empty: {}", mut_hello.is_empty()); // false

  println!("Contains \"World\": {}", mut_hello.contains("World")); // true
  
  println!("Replace \"World\": {}", mut_hello.replace("World", "There")); // Hello There!

  for word in mut_hello.split_whitespace() {
    println!("{}", word);
  }

  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  // assertion testing
  // assert_eq!(3, s.len());
    /*  thread 'main' panicked at 'assertion failed: `(left == right)`
        left: `3`, right: `2`', src/strings.rs:39:3
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace */
  assert_eq!(2, s.len());
  assert_eq!(11, s.capacity());

  println!("{}", s);

}