
use std::env;

pub fn run() {
  let args: Vec<String> = env::args().collect();
  let command = args[1].clone();
  let name = "Brad";
  let status = "100%";  

  println!("Args: {:?}", args);
  println!("Command: {:?}", command);

  if command == "hello" {
    println!("Hi {}, how are you?", name);
  } else if command == "status" {
    println!("Status is {}", status);
  } else {
    println!("{} is not a valid command", command);
  }
}

// 20/10/14 0:27:44 kvogel-macbook-2018:~/pl/learn-rust/crash-course/hello2 ±(master) ✗ 
// ❯ cargo run 1 2
//     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
//      Running `target/debug/hello2 1 2`
// Args: ["target/debug/hello2", "1", "2"]