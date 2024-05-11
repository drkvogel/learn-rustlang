
mod hello;
mod old;

use crate::hello::hello;

fn main() {
    println!("Hello, main!");
    hello::hello();
    hello();
    old::old();
    // std::fmt
}
