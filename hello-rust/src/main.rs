// use ferris_says::say;

use ferris_says::*;
use std::io::{ stdout, BufWriter };

fn main() {
  // say("Hello, world!")
  let stdout = stdout();
  // let out = b"Hello fellow Rustaceans!";
  // let width = out.len(); //  24;

  let message = String::from("Hello fellow Rustaceans!");
  let width = message.chars().count();
  
  let mut writer = BufWriter::new(stdout.lock());
  say(message.as_bytes(), width, &mut writer).unwrap();
}

// say(out, width, &mut writer).unwrap();

// fn main() {
//     println!("Hello, world!");
// }
