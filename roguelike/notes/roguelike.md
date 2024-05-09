
[Roguelike Tutorial in Rust + tcod ](https://tomassedovic.github.io/roguelike-tutorial/)
[rust - Why does `cargo new` create a binary instead of a library?](https://stackoverflow.com/questions/49706460/why-does-cargo-new-create-a-binary-instead-of-a-library)

[tomassedovic/tcod-rs: Rust bindings for libtcod 1.6.3 (the Doryen library/roguelike toolkit) ](https://github.com/tomassedovic/tcod-rs#how-to-use-this)

```toml
[dependencies]
tcod = "0.15"
```

```
20/10/24 4:56:44 kvogel-macbook-2018:~/Projects-learn/learn-rust/roguelike ±(master) ✗
❯ cargo run --release
    Blocking waiting for file lock on package cache
    Updating crates.io index
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling cc v1.0.61
   Compiling pkg-config v0.3.19
   Compiling bitflags v0.1.1
   Compiling lazy_static v0.1.16
   Compiling tcod-sys v5.0.1
   Compiling tcod v0.15.0
   Compiling roguelike v0.1.0 (/Users/kvogel/Projects-learn/learn-rust/roguelike)
    Finished release [optimized] target(s) in 35.98s
     Running `target/release/roguelike`
Hello, world!
```


`Blocking waiting for file lock on package cache` - because it was compiling dependencies?
[cargo run --release Blocking waiting for file lock on package cache](https://www.google.com/search?q=cargo+run+--release+Blocking+waiting+for+file+lock+on+package+cache&ie=UTF-8)
[rust - Cargo build hangs with " Blocking waiting for file lock on the registry index" after building parity from source](https://stackoverflow.com/questions/47565203/cargo-build-hangs-with-blocking-waiting-for-file-lock-on-the-registry-index-a)
[Cargo Package Cache is Locked? - help - The Rust Programming Language Forum ](https://users.rust-lang.org/t/cargo-package-cache-is-locked/34594)

```
20/10/24 5:01:43 kvogel-macbook-2018:~/Projects-learn/learn-rust/roguelike ±(master) ✗
❯ curl -O  https://github.com/tomassedovic/tcod-rs/raw/master/fonts/arial10x10.png
❯ lr
-rw-r--r--  1 kvogel  staff   148B 24 Oct 05:01 arial10x10.png

20/10/24 5:08:23 kvogel-macbook-2018:~/Projects-learn/learn-rust/roguelike ±(master) ✗
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/roguelike`
objc[82736]: Class SDLApplication is implemented in both /usr/local/opt/sdl2/lib/libSDL2-2.0.0.dylib (0x10bb3fa18) and /opt/local/lib/libSDL2-2.0.0.dylib (0x10ba19630). One of the two will be used. Which one is undefined.
objc[82736]: Class SDLAppDelegate is implemented in both /usr/local/opt/sdl2/lib/libSDL2-2.0.0.dylib (0x10bb3fa68) and /opt/local/lib/libSDL2-2.0.0.dylib (0x10ba19680). One of the two will be used. Which one is undefined.
objc[82736]: Class SDLTranslatorResponder is implemented in both /usr/local/opt/sdl2/lib/libSDL2-2.0.0.dylib (0x10bb3fae0) and /opt/local/lib/libSDL2-2.0.0.dylib (0x10ba196f8). One of the two will be used. Which one is undefined.
objc[82736]: Class SDLMessageBoxPresenter is implemented in both /usr/local/opt/sdl2/lib/libSDL2-2.0.0.dylib (0x10bb3fb08) and /opt/local/lib/libSDL2-2.0.0.dylib (0x10ba19720). One of the two will be used. Which one is undefined.
objc[82736]: Class SDL_cocoametalview is implemented in both /usr/local/opt/sdl2/lib/libSDL2-2.0.0.dylib (0x10bb3fb58) and /opt/local/lib/libSDL2-2.0.0.dylib (0x10ba19770). One of the two will be used. Which one is undefined.
objc[82736]: Class SDLOpenGLContext is implemented in both /usr/local/opt/sdl2/lib/libSDL2-2.0.0.dylib (0x10bb3fba8) and /opt/local/lib/libSDL2-2.0.0.dylib (0x10ba197c0). One of the two will be used. Which one is undefined.
objc[82736]: Class SDLWindow is implemented in both /usr/local/opt/sdl2/lib/libSDL2-2.0.0.dylib (0x10bb3fbf8) and /opt/local/lib/libSDL2-2.0.0.dylib (0x10ba19810). One of the two will be used. Which one is undefined.
objc[82736]: Class Cocoa_WindowListener is implemented in both /usr/local/opt/sdl2/lib/libSDL2-2.0.0.dylib (0x10bb3fc20) and /opt/local/lib/libSDL2-2.0.0.dylib (0x10ba19838). One of the two will be used. Which one is undefined.
objc[82736]: Class SDLView is implemented in both /usr/local/opt/sdl2/lib/libSDL2-2.0.0.dylib (0x10bb3fc98) and /opt/local/lib/libSDL2-2.0.0.dylib (0x10ba198b0). One of the two will be used. Which one is undefined.
objc[82736]: Class METAL_RenderData is implemented in both /usr/local/opt/sdl2/lib/libSDL2-2.0.0.dylib (0x10bb3fce8) and /opt/local/lib/libSDL2-2.0.0.dylib (0x10ba19900). One of the two will be used. Which one is undefined.
objc[82736]: Class METAL_TextureData is implemented in both /usr/local/opt/sdl2/lib/libSDL2-2.0.0.dylib (0x10bb3fd38) and /opt/local/lib/libSDL2-2.0.0.dylib (0x10ba19950). One of the two will be used. Which one is undefined.
libtcod 1.6.3
SDL : cannot load arial10x10.png

❯ ll
total 24
-rw-r--r--  1 kvogel  staff   1.4K 24 Oct 04:56 Cargo.lock
-rw-r--r--  1 kvogel  staff   240B 24 Oct 04:56 Cargo.toml
-rw-r--r--  1 kvogel  staff   148B 24 Oct 05:01 arial10x10.png
drwxr-xr-x  3 kvogel  staff    96B 24 Oct 04:54 notes/
drwxr-xr-x  3 kvogel  staff    96B 24 Oct 05:08 src/
drwxr-xr-x@ 7 kvogel  staff   224B 24 Oct 04:56 target/

❯ file arial10x10.png
arial10x10.png: HTML document text, ASCII text, with no line terminators
```
>you might check whether arial10x10.png is actually an image; libtcod gives the same error message whether the file doesn't exist or it just can't be read.  When I first did the tutorial, I accidentally downloaded the github html page for the image instead of the image itself.

>The -L flag instructs cURL to follow any redirect so that you reach the eventual endpoint.
```
20/10/24 5:09:27 kvogel-macbook-2018:~/Projects-learn/learn-rust/roguelike ±(master) ✗
❯ curl -OL  https://github.com/tomassedovic/tcod-rs/raw/master/fonts/arial10x10.png
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   148  100   148    0     0    355      0 --:--:-- --:--:-- --:--:--   354
100 14870  100 14870    0     0  20314      0 --:--:-- --:--:-- --:--:-- 20314

❯ file arial10x10.png
arial10x10.png: PNG image data, 320 x 80, 8-bit/color RGB, non-interlaced
```

```
20/10/24 5:11:14 kvogel-macbook-2018:~/Projects-learn/learn-rust/roguelike ±(master) ✗
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/roguelike`
```
get a black window with an `@` in the corner. Cool!


### Class SDLApplication is implemented in both...
```
Class SDLApplication is implemented in both /usr/local/opt/sdl2/lib/libSDL2-2.0.0.dylib (0x10bb3fa18) and /opt/local/lib/libSDL2-2.0.0.dylib (0x10ba19630). One of the two will be used. Which one is undefined.
```

search "Class SDLApplication is implemented in both  One of the two will be used. Which one is undefined."

doesn't prevent compilation, though.
