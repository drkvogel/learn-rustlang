
[Rust Crash Course  Rustlang](https://www.youtube.com/watch?v=zF34dRivLOw)
rustup
rustc
cargo

```
20/10/12 21:03:52 kvogel-macbook-2018:~/Projects-learn/learn-rust/crash-course ±(master) ✗ 
❯ cargo new hello
     Created binary (application) `hello` package
❯ ls
hello/
❯ tree hello 
hello
├── Cargo.toml
└── src
    └── main.rs
```
or
```
20/10/12 21:12:59 kvogel-macbook-2018:~/pl/learn-rust/crash-course ±(master) ✗ 
❯ mkdir hello2
❯ cd hello2
❯ cargo init
     Created binary (application) package`
❯ tree
.
├── Cargo.toml
└── src
    └── main.rs
```


```
20/10/12 21:15:52 kvogel-macbook-2018:~/pl/learn-rust/crash-course/hello2 ±(master) ✗ 
❯ cargo run
   Compiling hello2 v0.1.0 (/Users/kvogel/Projects-learn/learn-rust/crash-course/hello2)
    Finished dev [unoptimized + debuginfo] target(s) in 4.88s
     Running `target/debug/hello2`
Hello, world!

20/10/12 21:17:06 kvogel-macbook-2018:~/pl/learn-rust/crash-course/hello2 ±(master) ✗ 
❯ tree
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    └── debug
        ├── build
        ├── deps
        │   ├── hello2-1c951bc41d73cdf1
        │   ├── hello2-1c951bc41d73cdf1.d
        │   └── hello2-1c951bc41d73cdf1.dSYM
        │       └── Contents
        │           ├── Info.plist
        │           └── Resources
        │               └── DWARF
        │                   └── hello2-1c951bc41d73cdf1
        ├── examples
        ├── hello2
        ├── hello2.d
        ├── hello2.dSYM -> deps/hello2-1c951bc41d73cdf1.dSYM
        └── incremental
            └── hello2-2n3j7jd8jhsw1
                ├── s-fs1tv1ryq3-1pnb8n4-8oyy2a72rr1
                │   ├── 1l2q31rxacznhnpp.o
                │   ├── 1m1ylnyzburo3avc.o
                │   ├── 2mafz48vefv4sh52.o
                │   ├── 2pu3b3jnggzdsggs.o
                │   ├── 2t6jq7ppzr2cni28.o
                │   ├── 5221xzxjb0igits1.o
                │   ├── dep-graph.bin
                │   ├── query-cache.bin
                │   └── work-products.bin
                └── s-fs1tv1ryq3-1pnb8n4.lock

14 directories, 19 files
```
run (debug) executable directly:
```
❯ ./target/debug/hello2
Hello, world!
```
just build, don't run:
```
❯ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
```
build (optimized) production/release build:
```
❯ cargo build --release
   Compiling hello2 v0.1.0 (/Users/kvogel/Projects-learn/learn-rust/crash-course/hello2)
    Finished release [optimized] target(s) in 0.85s
❯ ./target/release/hello2 
Hello, world!
```

### println and modules

`print.rs`:
```rs
pub fn run() {
  // print to console
  println!("Hello from print.rs file")
}
```
`main.rs`:
```rs
mod print;

fn main() {
    // println!("Hello, world!");
    print::run();
}
```

```
20/10/12 21:32:26 kvogel-macbook-2018:~/pl/learn-rust/crash-course/hello2 ±(master) ✗ 
❯ cargo build          
   Compiling hello2 v0.1.0 (/Users/kvogel/Projects-learn/learn-rust/crash-course/hello2)
    Finished dev [unoptimized + debuginfo] target(s) in 1.18s
20/10/12 21:32:35 kvogel-macbook-2018:~/pl/learn-rust/crash-course/hello2 ±(master) ✗ 
❯ ./target/release/hello2    
Hello, world!
20/10/12 21:32:48 kvogel-macbook-2018:~/pl/learn-rust/crash-course/hello2 ±(master) ✗ 
❯ ./target/debug/hello2      
Hello from print.rs file
```

[vscode setup](/notes/vscode.md)

