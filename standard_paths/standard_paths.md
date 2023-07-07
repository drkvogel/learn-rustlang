


### Rust Standard Paths crate

In `bevy/examples`, cloned a game `github.com:gardum-game/gardum` and tried to run it, compile errored with:

>error[E0599]: no method named `standard_locations_impl` found for reference `&StandardPaths` in the current scope

```
23/05/2 20:31:26 kvogel@kvogel-macbook-2021:~/projects/general/dev/learn/learn-rustlang/bevy/examples ±(master)
❯ git clone git@github.com:gardum-game/gardum.git

23/05/2 20:31:33 kvogel@kvogel-macbook-2021:~/projects/general/dev/learn/learn-rustlang/bevy/examples/gardum ±(master)
❯ cargo run --release
...
   Compiling clap v3.2.14
error[E0599]: no method named `writable_location_impl` found for reference `&StandardPaths` in the current scope
   --> /Users/kvogel/.cargo/registry/src/github.com-1ecc6299db9ec823/standard_paths-1.1.0/src/lib.rs:201:14
    |
201 |         self.writable_location_impl(location)
    |              ^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `writable_location`

error[E0599]: no method named `standard_locations_impl` found for reference `&StandardPaths` in the current scope
   --> /Users/kvogel/.cargo/registry/src/github.com-1ecc6299db9ec823/standard_paths-1.1.0/src/lib.rs:216:14
    |
216 |         self.standard_locations_impl(location)
    |              ^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `standard_locations`

error[E0425]: cannot find function `find_executable_in_paths_impl` in this scope
   --> /Users/kvogel/.cargo/registry/src/github.com-1ecc6299db9ec823/standard_paths-1.1.0/src/lib.rs:265:9
    |
265 |         find_executable_in_paths_impl(name, &paths)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope

Some errors have detailed explanations: E0425, E0599.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `standard_paths` due to 3 previous errors
```

In the [README](file:///Users/kvogel/.cargo/registry/src/github.com-1ecc6299db9ec823/standard_paths-1.1.0/README.md):
>Currently implemented for Linux and Windows
>macOS is currently unsupported. If you want to help with macOS feel free to contribute!


[Add macOS support · Issue #2 · mentaljam/standard_paths ](https://github.com/mentaljam/standard_paths/issues/2#issuecomment-1532527330)
>mentaljam commented 2 hours ago
>Hello and thanks for your reply! I ported this library for another (currently abandoned) project. But it seems useful to someone, as it continues to be downloaded on crates.io :) If you want to implement support for macOS, you can start from the original QStandardPaths and port the appropriate part of the code. I used version 5.x, but I think you can start with version 6.x.

[QStandardPaths Class  Qt Core 5.15.13 ](https://doc.qt.io/qt-5/qstandardpaths.html#details)
[standard_paths - Rust ](https://docs.rs/standard_paths/latest/standard_paths/)
[standard_paths - crates.io: Rust Package Registry ](https://crates.io/crates/standard_paths)
[mentaljam/standard_paths: A Rust port of QStandardPaths class which provides methods for accessing standard paths on the local filesystem (config, cache, user directories and etc.). ](https://github.com/mentaljam/standard_paths)

[Add macOS support · Issue #2 · mentaljam/standard_paths ](https://github.com/mentaljam/standard_paths/issues/2)

[diff](/diff.md) between [github repo](/standard_paths/README.md) and [cargo-installed lib](file:///Users/kvogel/.cargo/registry/src/github.com-1ecc6299db9ec823/standard_paths-1.1.0/README.md)



```
23/05/2 20:31:33 kvogel@kvogel-macbook-2021:~/projects/general/dev/learn/learn-rustlang/bevy/examples/gardum ±(master)
❯ cargo run --release
...
   Compiling standard_paths v1.1.0
   Compiling clap v3.2.14
error[E0599]: no method named `writable_location_impl` found for reference `&StandardPaths` in the current scope
   --> /Users/kvogel/.cargo/registry/src/github.com-1ecc6299db9ec823/standard_paths-1.1.0/src/lib.rs:201:14
    |
201 |         self.writable_location_impl(location)
    |              ^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `writable_location`

error[E0599]: no method named `standard_locations_impl` found for reference `&StandardPaths` in the current scope
   --> /Users/kvogel/.cargo/registry/src/github.com-1ecc6299db9ec823/standard_paths-1.1.0/src/lib.rs:216:14
    |
216 |         self.standard_locations_impl(location)
    |              ^^^^^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `standard_locations`

error[E0425]: cannot find function `find_executable_in_paths_impl` in this scope
   --> /Users/kvogel/.cargo/registry/src/github.com-1ecc6299db9ec823/standard_paths-1.1.0/src/lib.rs:265:9
    |
265 |         find_executable_in_paths_impl(name, &paths)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope

Some errors have detailed explanations: E0425, E0599.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `standard_paths` due to 3 previous errors
```
