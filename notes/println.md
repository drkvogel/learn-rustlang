
what happens if I don't put `!` after `println`?
```rs
fn main() {
  // println!("Hello World!")
  println("Hello World!")
}
```
```
20/10/12 20:48:41 kvogel-macbook-2018:~/Projects-learn/learn-rust/hello_world ±(master) ✗ 
❯ rustc hello.rs
error[E0423]: expected function, found macro `println`
```
```rs
 --> hello.rs:3:3
  |
3 |   println("Hello World!")
  |   ^^^^^^^
  |
```
```
help: use `!` to invoke the macro

For more information about this error, try `rustc --explain E0423`.

An identifier was used like a function name or a value was expected and the
identifier exists but it belongs to a different namespace.

Erroneous code example:
```
```rs
struct Foo { a: bool };

let f = Foo();
// error: expected function, tuple struct or tuple variant, found `Foo`
// `Foo` is a struct name, but this expression uses it like a function name
```
```
It is common to forget the trailing `!` on macro invocations, which would also
yield this error
```


[newb question: why is println! a macro? : rust ](https://www.reddit.com/r/rust/comments/4qor4o/newb_question_why_is_println_a_macro/)

println!() does a couple of things that a regular function can't do:

It parses the format string at compile time, and generates type safe code
It has a variable number of arguments
It has named arguments ("keyword arguments")
It takes parameters by reference implicitly
