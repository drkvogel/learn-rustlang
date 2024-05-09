
mod hello;

use crate::hello::hello;

fn main() {
    println!("Hello, main!");
    hello::hello();
    hello();
    // std::fmt
}
