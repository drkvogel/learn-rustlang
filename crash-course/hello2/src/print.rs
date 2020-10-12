pub fn run() {
  // print to console
  // println!("Hello from print.rs file");
  println!("Number: {}", 1);

  // basic formatting
  println!("{} lives in {}", "Chris", "Oxford");
  
  // positional arguments
  println!("{0} lives in {1} and {0} likes to {2}.", "Chris", "Oxford", "code"); // Chris lives in Oxford and Chris likes to code.
  
  // named arguments
  println!("{name} lives in {place}", name = "Remi", place = "Tackley"); // Remi lives in Tackley
  
  // placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal {:o}", 10, 10, 10); // Binary: 1010 Hex: a Octal 12

  // debug trait
  println!("{:?}", (12, true, "Hello")); // (12, true, "Hello")

  // basic maths
  println!("10 + 10 = {}", 10 + 10);
}