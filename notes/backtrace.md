

### `RUST_BACKTRACE=1`

```
20/10/13 0:23:42 kvogel-macbook-2018:~/pl/learn-rust/crash-course/hello2 ±(master) ✗ 
❯ RUST_BACKTRACE=1 cargo run
   Compiling hello2 v0.1.0 (/Users/kvogel/Projects-learn/learn-rust/crash-course/hello2)
    Finished dev [unoptimized + debuginfo] target(s) in 1.31s
     Running `target/debug/hello2`
Hello
Length of hello: String: 5
Length: literal: str: 7
Hello W
Hello World!
Capacity: 12
Is empty: false
Contains "World": true
Replace "World": Hello There!
Hello
World!
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `3`,
 right: `2`', src/strings.rs:39:3
stack backtrace:
   0: rust_begin_unwind
             at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/panicking.rs:475
   1: std::panicking::begin_panic_fmt
             at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/panicking.rs:429
   2: hello2::strings::run
             at ./src/strings.rs:39
   3: hello2::main
             at ./src/main.rs:11
   4: core::ops::function::FnOnce::call_once
             at /Users/kvogel/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

### `RUST_BACKTRACE=full`

```
20/10/13 0:25:08 kvogel-macbook-2018:~/pl/learn-rust/crash-course/hello2 ±(master) ✗ 
❯ RUST_BACKTRACE=full cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/hello2`
Hello
Length of hello: String: 5
Length: literal: str: 7
Hello W
Hello World!
Capacity: 12
Is empty: false
Contains "World": true
Replace "World": Hello There!
Hello
World!
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `3`,
 right: `2`', src/strings.rs:39:3
stack backtrace:
   0:        0x104de32b4 - std::backtrace_rs::backtrace::libunwind::trace::h93742dd1b3bd2027
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/../../backtrace/src/backtrace/libunwind.rs:96
   1:        0x104de32b4 - std::backtrace_rs::backtrace::trace_unsynchronized::h811523dd305c9cc6
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/../../backtrace/src/backtrace/mod.rs:66
   2:        0x104de32b4 - std::sys_common::backtrace::_print_fmt::h648a1661b958f275
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/sys_common/backtrace.rs:79
   3:        0x104de32b4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd4b962ed89f71a03
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/sys_common/backtrace.rs:58
   4:        0x104df9510 - core::fmt::write::h94ae1e793baa7a00
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/core/src/fmt/mod.rs:1082
   5:        0x104de158b - std::io::Write::write_fmt::h5c716758fdc3057f
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/io/mod.rs:1514
   6:        0x104de4b85 - std::sys_common::backtrace::_print::he99526f3d58c4f6f
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/sys_common/backtrace.rs:61
   7:        0x104de4b85 - std::sys_common::backtrace::print::h18711c06562fe387
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/sys_common/backtrace.rs:48
   8:        0x104de4b85 - std::panicking::default_hook::{{closure}}::hc6119c7d16548caf
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/panicking.rs:200
   9:        0x104de48c7 - std::panicking::default_hook::heae8b62897b351dc
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/panicking.rs:219
  10:        0x104de5125 - std::panicking::rust_panic_with_hook::hc36596b4257bea99
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/panicking.rs:569
  11:        0x104de4cbb - std::panicking::begin_panic_handler::{{closure}}::h49e5ddc3f21ca2fb
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/panicking.rs:476
  12:        0x104de3728 - std::sys_common::backtrace::__rust_end_short_backtrace::h9bd32c9ad3fad18f
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/sys_common/backtrace.rs:153
  13:        0x104de4c7a - rust_begin_unwind
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/panicking.rs:475
  14:        0x104dfe1fb - std::panicking::begin_panic_fmt::hc5bc7b154cdecca6
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/panicking.rs:429
  15:        0x104dcd8e6 - hello2::strings::run::h28cff8c27e035c2e
                               at /Users/kvogel/Projects-learn/learn-rust/crash-course/hello2/src/strings.rs:39
  16:        0x104dcb629 - hello2::main::h51735997ece93f47
                               at /Users/kvogel/Projects-learn/learn-rust/crash-course/hello2/src/main.rs:11
  17:        0x104dc738e - core::ops::function::FnOnce::call_once::h15603a9e82dbb3ea
                               at /Users/kvogel/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:227
  18:        0x104dc7c61 - std::sys_common::backtrace::__rust_begin_short_backtrace::h9f9397dfcef67583
                               at /Users/kvogel/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys_common/backtrace.rs:137
  19:        0x104dca974 - std::rt::lang_start::{{closure}}::hdabddd18220d69e5
                               at /Users/kvogel/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/rt.rs:66
  20:        0x104de5400 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::h8a9b574793342aba
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/core/src/ops/function.rs:259
  21:        0x104de5400 - std::panicking::try::do_call::h4f96def3e784cc72
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/panicking.rs:373
  22:        0x104de5400 - std::panicking::try::h852a41061a270947
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/panicking.rs:337
  23:        0x104de5400 - std::panic::catch_unwind::h1c1a28455dde417c
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/panic.rs:379
  24:        0x104de5400 - std::rt::lang_start_internal::hd0c760d47f593c0a
                               at /rustc/18bf6b4f01a6feaf7259ba7cdae58031af1b7b39/library/std/src/rt.rs:51
  25:        0x104dca951 - std::rt::lang_start::h08523d2f298fdbf7
                               at /Users/kvogel/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/rt.rs:65
  26:        0x104dcb652 - _main
```

