
use std::env;

pub fn run() {
  let user = env::var("USERNAME").unwrap();
  if user == "kvogel" {
    println!("Guten Abend, Herr Vogel ({})", user);
  } else {
    println!("Good day, {}", user);
  }
}

/*

?? USERNAME is set:

```
20/10/20 15:53:25 kvogel-macbook-2018:~/Projects-learn/learn-rust/learn ±(master) ✗ 
❯ echo $USERNAME
kvogel
```

, but the program panics when trying to unwrap() (?) env::var("USERNAME"):

```
❯ ./target/debug/learn    
Hello, world!
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: NotPresent', src/unwrap.rs:5:35
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

set USERNAME on the command line

20/10/20 15:52:38 kvogel-macbook-2018:~/Projects-learn/learn-rust/learn ±(master) ✗ 
❯ USERNAME=blah ./target/debug/learn
Hello, world!
Guten Abend, Herr Vogel (kvogel)



❯ export TEST=foo
❯ export TEST=foo; echo $TEST
foo


20/10/20 15:53:46 kvogel-macbook-2018:~/Projects-learn/learn-rust/learn ±(master) ✗ 
❯ RUST_BACKTRACE=1 ./target/debug/learn
Hello, world!
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: NotPresent', src/unwrap.rs:5:35
stack backtrace:
   0: rust_begin_unwind
             at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/panicking.rs:475
   1: core::panicking::panic_fmt
             at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/core/src/panicking.rs:85
   2: core::option::expect_none_failed
             at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/core/src/option.rs:1221
   3: core::result::Result<T,E>::unwrap
             at /Users/kvogel/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:973
   4: learn::unwrap::run
             at ./src/unwrap.rs:5
   5: learn::main
             at ./src/main.rs:8
   6: core::ops::function::FnOnce::call_once
             at /Users/kvogel/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
*/