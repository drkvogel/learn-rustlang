
[Intro to web programming in Rust for NodeJS developers ](https://levelup.gitconnected.com/intro-to-web-programming-in-rust-for-nodejs-developers-1a9c048c4de1)
[oxide rustlang](https://www.google.com/search?rlz=1C5CHFA_enGB838GB838&ei=5UuVXL2oNumW1fAP7qC84Ac&q=oxide+rustlang&oq=oxide+rustlang&gs_l=psy-ab.3..0i8i13i30l3.44010.44521..45038...0.0..0.164.642.0j4......0....1..gws-wiz.......0i71j35i304i39j0i13.CSig23dskn0)
[Oxide: A Formal Semantics for Rust  Hacker News ](https://news.ycombinator.com/item?id=19310950)
[rust lang](https://www.google.com/search?q=rust+lang&rlz=1C5CHFA_enGB838GB838&oq=rust+lang&aqs=chrome..69i57.1355j0j7&sourceid=chrome&ie=UTF-8)
[Rust programming language ](https://www.rust-lang.org/)


### Installing gitui from source

[extrawurst/gitui: Blazing 💥 fast terminal-ui for git written in rust 🦀 ](https://github.com/extrawurst/gitui)
```
21/02/4 10:02:36 kvogel-elitebook:~/po
❯ pwd -P
/home/kvogel/projects/other

❯ git clone https://github.com/extrawurst/gitui
❯ cd gitui
❯ ls
CHANGELOG.md  Cargo.toml     LICENSE.md  README.md  assets/    invalidstring/  scopetime/  wix/
Cargo.lock    KEY_CONFIG.md  Makefile    THEMES.md  asyncgit/  rustfmt.toml    src/
❯ which cargo
/home/kvogel/.cargo/bin/cargo
❯ cargo build
error: no override and no default toolchain set
❯ rustc
error: no override and no default toolchain set
```
[rust - No default toolchain configured after installing rustup](https://stackoverflow.com/questions/44303915/no-default-toolchain-configured-after-installing-rustup)
```
❯ rustup install stable
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
info: latest update on 2020-12-31, rust version 1.49.0 (e1884a8e3 2020-12-29)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
 13.8 MiB /  13.8 MiB (100 %)   8.7 MiB/s in  1s ETA:  0s
info: downloading component 'rust-std'
 22.3 MiB /  22.3 MiB (100 %)   8.7 MiB/s in  2s ETA:  0s
info: downloading component 'rustc'
 55.7 MiB /  55.7 MiB (100 %)   8.4 MiB/s in  7s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: using up to 500.0 MiB of RAM to unpack components
  5.3 MiB /   5.3 MiB (100 %)   4.3 MiB/s in  1s ETA:  0s
info: installing component 'clippy'
info: installing component 'rust-docs'
 13.8 MiB /  13.8 MiB (100 %)   1.1 MiB/s in 13s ETA:  0s
info: installing component 'rust-std'
 22.3 MiB /  22.3 MiB (100 %)   1.7 MiB/s in 10s ETA:  0s
info: installing component 'rustc'
 55.7 MiB /  55.7 MiB (100 %)   2.7 MiB/s in 48s ETA:  0s
info: installing component 'rustfmt'

  stable-x86_64-unknown-linux-gnu installed - rustc 1.49.0 (e1884a8e3 2020-12-29)

info: default toolchain set to 'stable-x86_64-unknown-linux-gnu'
info: checking for self-updates
21/02/4 10:06:17 kvogel-elitebook:~/po/gitui ±(master)
❯ rustc
Usage: rustc [OPTIONS] INPUT

Options:
    -h, --help          Display this message
        --cfg SPEC      Configure the compilation environment
    -L [KIND=]PATH      Add a directory to the library search path. The
                        optional KIND can be one of dependency, crate, native,
                        framework, or all (the default).
    -l [KIND=]NAME      Link the generated crate(s) to the specified native
                        library NAME. The optional KIND can be one of
                        static, framework, or dylib (the default).
        --crate-type [bin|lib|rlib|dylib|cdylib|staticlib|proc-macro]
                        Comma separated list of types of crates
                        for the compiler to emit
        --crate-name NAME
                        Specify the name of the crate being built
        --edition 2015|2018
                        Specify which edition of the compiler to use when
                        compiling code.
        --emit [asm|llvm-bc|llvm-ir|obj|metadata|link|dep-info|mir]
                        Comma separated list of types of output for the
                        compiler to emit
        --print [crate-name|file-names|sysroot|target-libdir|cfg|target-list|target-cpus|target-features|relocation-models|code-models|tls-models|target-spec-json|native-static-libs]
                        Compiler information to print on stdout
    -g                  Equivalent to -C debuginfo=2
    -O                  Equivalent to -C opt-level=2
    -o FILENAME         Write output to <filename>
        --out-dir DIR   Write output to compiler-chosen filename in <dir>
        --explain OPT   Provide a detailed explanation of an error message
        --test          Build a test harness
        --target TARGET Target triple for which the code is compiled
    -W, --warn OPT      Set lint warnings
    -A, --allow OPT     Set lint allowed
    -D, --deny OPT      Set lint denied
    -F, --forbid OPT    Set lint forbidden
        --cap-lints LEVEL
                        Set the most restrictive lint level. More restrictive
                        lints are capped at this level
    -C, --codegen OPT[=VALUE]
                        Set a codegen option
    -V, --version       Print version info and exit
    -v, --verbose       Use verbose output

Additional help:
    -C help             Print codegen options
    -W help             Print 'lint' options and default settings
    --help -v           Print the full set of options rustc accepts

21/02/4 10:15:49 kvogel-elitebook:~/po/gitui ±(master)
❯ rustup install stable
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.49.0 (e1884a8e3 2020-12-29)

info: checking for self-updates
21/02/4 10:15:51 kvogel-elitebook:~/po/gitui ±(master)21/02/4 10:15:51 kvogel-elitebook:~/po/gitui ±(master)
❯ cargo build
    Updating crates.io index
  Downloaded time v0.1.44
  Downloaded const_fn v0.4.3
  Downloaded addr2line v0.14.1
  Downloaded openssl-sys v0.9.58
  Downloaded percent-encoding v2.1.0
  Downloaded unicode-normalization v0.1.16
  Downloaded tinyvec_macros v0.1.0
  Downloaded simplelog v0.9.0
  Downloaded crossbeam-deque v0.8.0
  Downloaded chrono v0.4.19
  Downloaded scopeguard v1.1.0
  Downloaded parking_lot_core v0.8.1
  Downloaded thiserror-impl v1.0.23
  Downloaded thiserror v1.0.23
  Downloaded num-traits v0.2.14
  Downloaded mio v0.7.6
  Downloaded log v0.4.14
  Downloaded dirs-next v2.0.0
  Downloaded syn v1.0.60
  Downloaded tinyvec v1.1.0
  Downloaded smawk v0.3.0
  Downloaded textwrap v0.13.2
  Downloaded ron v0.6.4
  Downloaded serde_derive v1.0.123
  Downloaded signal-hook-registry v1.2.2
  Downloaded signal-hook v0.1.16
  Downloaded instant v0.1.9
  Downloaded which v4.0.2
  Downloaded parking_lot v0.11.1
  Downloaded form_urlencoded v1.0.0
  Downloaded idna v0.2.0
  Downloaded rayon-core v1.9.0
  Downloaded object v0.23.0
  Downloaded gimli v0.23.0
  Downloaded matches v0.1.8
  Downloaded adler v0.2.3
  Downloaded itertools v0.10.0
  Downloaded cc v1.0.66
  Downloaded serde v1.0.123
  Downloaded unicode-bidi v0.3.4
  Downloaded unicode-segmentation v1.7.1
  Downloaded smallvec v1.5.1
  Downloaded num-integer v0.1.44
  Downloaded lock_api v0.4.2
  Downloaded rustc-demangle v0.1.18
  Downloaded memoffset v0.6.1
  Downloaded either v1.6.1
  Downloaded miniz_oxide v0.4.3
  Downloaded tui v0.14.0
  Downloaded dirs-sys-next v0.1.1
  Downloaded jobserver v0.1.21
  Downloaded url v2.2.0
  Downloaded crossterm v0.19.0
  Downloaded git2 v0.13.17
  Downloaded libssh2-sys v0.2.20
  Downloaded crossterm v0.18.2
  Downloaded crossbeam-epoch v0.9.1
  Downloaded bytesize v1.0.1
  Downloaded crossbeam-channel v0.5.0
  Downloaded pkg-config v0.3.19
  Downloaded anyhow v1.0.38
  Downloaded cassowary v0.3.0
  Downloaded base64 v0.13.0
  Downloaded openssl-probe v0.1.2
  Downloaded backtrace v0.3.56
  Downloaded libgit2-sys v0.12.18+1.1.0
  Downloaded libz-sys v1.1.2
  Downloaded openssl-src v111.12.0+1.1.1h
  Downloaded 68 crates (12.3 MB) in 2.18s (largest was `openssl-src` at 5.1 MB)
   Compiling libc v0.2.81
   Compiling autocfg v1.0.1
   Compiling cfg-if v1.0.0
   Compiling pkg-config v0.3.19
   Compiling proc-macro2 v1.0.24
   Compiling unicode-xid v0.2.1
   Compiling log v0.4.14
   Compiling syn v1.0.60
   Compiling scopeguard v1.1.0
   Compiling bitflags v1.2.1
   Compiling lazy_static v1.4.0
   Compiling serde_derive v1.0.123
   Compiling tinyvec_macros v0.1.0
   Compiling const_fn v0.4.3
   Compiling matches v0.1.8
   Compiling smallvec v1.5.1
   Compiling serde v1.0.123
   Compiling percent-encoding v2.1.0
   Compiling unicode-width v0.1.8
   Compiling rayon-core v1.9.0
   Compiling gimli v0.23.0
   Compiling openssl-probe v0.1.2
   Compiling adler v0.2.3
   Compiling anyhow v1.0.38
   Compiling smawk v0.3.0
   Compiling either v1.6.1
   Compiling cassowary v0.3.0
   Compiling unicode-segmentation v1.7.1
   Compiling rustc-demangle v0.1.18
   Compiling base64 v0.13.0
   Compiling object v0.23.0
   Compiling bytesize v1.0.1
   Compiling instant v0.1.9
   Compiling crossbeam-utils v0.8.1
   Compiling memoffset v0.6.1
   Compiling num-traits v0.2.14
   Compiling num-integer v0.1.44
   Compiling miniz_oxide v0.4.3
   Compiling lock_api v0.4.2
   Compiling tinyvec v1.1.0
   Compiling unicode-bidi v0.3.4
   Compiling form_urlencoded v1.0.0
   Compiling textwrap v0.11.0
   Compiling textwrap v0.13.2
   Compiling itertools v0.10.0
   Compiling addr2line v0.14.1
   Compiling unicode-normalization v0.1.16
   Compiling jobserver v0.1.21
   Compiling signal-hook-registry v1.2.2
   Compiling parking_lot_core v0.8.1
   Compiling time v0.1.44
   Compiling num_cpus v1.13.0
   Compiling dirs-sys-next v0.1.1
   Compiling quote v1.0.7
   Compiling clap v2.33.3
   Compiling mio v0.7.6
   Compiling scopetime v0.1.1 (/home/kvogel/projects/other/gitui/scopetime)
   Compiling crossbeam-channel v0.5.0
   Compiling idna v0.2.0
   Compiling cc v1.0.66
   Compiling crossbeam-epoch v0.9.1
   Compiling parking_lot v0.11.1
   Compiling dirs-next v2.0.0
   Compiling signal-hook v0.1.16
   Compiling backtrace v0.3.56
   Compiling url v2.2.0
   Compiling crossbeam-deque v0.8.0
   Compiling openssl-src v111.12.0+1.1.1h
   Compiling chrono v0.4.19
   Compiling crossterm v0.18.2
   Compiling libz-sys v1.1.2
   Compiling libssh2-sys v0.2.20
   Compiling libgit2-sys v0.12.18+1.1.0
   Compiling openssl-sys v0.9.58
   Compiling simplelog v0.9.0
   Compiling thiserror-impl v1.0.23
   Compiling thiserror v1.0.23
   Compiling which v4.0.2
   Compiling crossterm v0.19.0
   Compiling ron v0.6.4
   Compiling tui v0.14.0
   Compiling git2 v0.13.17
   Compiling asyncgit v0.11.0 (/home/kvogel/projects/other/gitui/asyncgit)
   Compiling gitui v0.11.0 (/home/kvogel/projects/other/gitui)
    Finished dev [unoptimized + debuginfo] target(s) in 6m 59s
21/02/4 10:22:54 kvogel-elitebook:~/po/gitui ±(master)
❯
21/02/4 10:24:58 kvogel-elitebook:~/po/gitui ±(master)
❯ gitui
zsh: command not found: gitui
21/02/4 10:25:04 kvogel-elitebook:~/po/gitui ±(master)
❯ cargo install gitui
    Updating crates.io index
  Downloaded gitui v0.11.0
  Downloaded 1 crate (9.5 MB) in 3.02s
  Installing gitui v0.11.0
  Downloaded signal-hook-registry v1.3.0
  Downloaded dirs-sys-next v0.1.2
  Downloaded asyncgit v0.11.0
  Downloaded smallvec v1.6.1
  Downloaded signal-hook v0.1.17
  Downloaded parking_lot_core v0.8.2
  Downloaded mio v0.7.7
  Downloaded libc v0.2.85
  Downloaded simplelog v0.8.0
  Downloaded time v0.1.43
  Downloaded tinyvec v1.1.1
  Downloaded quote v1.0.8
  Downloaded openssl-sys v0.9.60
  Downloaded itertools v0.9.0
  Downloaded const_fn v0.4.5
  Downloaded scopetime v0.1.1
  Downloaded tui v0.13.0
  Downloaded libssh2-sys v0.2.21
  Downloaded openssl-src v111.13.0+1.1.1i
  Downloaded smawk v0.3.1
  Downloaded 20 crates (6.7 MB) in 1.36s (largest was `openssl-src` at 5.1 MB)
   Compiling libc v0.2.85
   Compiling autocfg v1.0.1
   Compiling cfg-if v1.0.0
   Compiling pkg-config v0.3.19
   Compiling proc-macro2 v1.0.24
   Compiling unicode-xid v0.2.1
   Compiling syn v1.0.60
   Compiling log v0.4.14
   Compiling bitflags v1.2.1
   Compiling scopeguard v1.1.0
   Compiling lazy_static v1.4.0
   Compiling tinyvec_macros v0.1.0
   Compiling matches v0.1.8
   Compiling const_fn v0.4.5
   Compiling serde_derive v1.0.123
   Compiling serde v1.0.123
   Compiling percent-encoding v2.1.0
   Compiling smallvec v1.6.1
   Compiling unicode-width v0.1.8
   Compiling rayon-core v1.9.0
   Compiling anyhow v1.0.38
   Compiling openssl-probe v0.1.2
   Compiling adler v0.2.3
   Compiling gimli v0.23.0
   Compiling base64 v0.13.0
   Compiling rustc-demangle v0.1.18
   Compiling unicode-segmentation v1.7.1
   Compiling cassowary v0.3.0
   Compiling either v1.6.1
   Compiling object v0.23.0
   Compiling smawk v0.3.1
   Compiling bytesize v1.0.1
   Compiling instant v0.1.9
   Compiling crossbeam-utils v0.8.1
   Compiling memoffset v0.6.1
   Compiling num-traits v0.2.14
   Compiling num-integer v0.1.44
   Compiling miniz_oxide v0.4.3
   Compiling lock_api v0.4.2
   Compiling tinyvec v1.1.1
   Compiling unicode-bidi v0.3.4
   Compiling form_urlencoded v1.0.0
   Compiling textwrap v0.11.0
   Compiling itertools v0.9.0
   Compiling textwrap v0.13.2
   Compiling addr2line v0.14.1
   Compiling unicode-normalization v0.1.16
   Compiling signal-hook-registry v1.3.0
   Compiling parking_lot_core v0.8.2
   Compiling num_cpus v1.13.0
   Compiling time v0.1.43
   Compiling dirs-sys-next v0.1.2
   Compiling jobserver v0.1.21
   Compiling clap v2.33.3
   Compiling mio v0.7.7
   Compiling scopetime v0.1.1
   Compiling quote v1.0.8
   Compiling idna v0.2.0
   Compiling parking_lot v0.11.1
   Compiling dirs-next v2.0.0
   Compiling cc v1.0.66
   Compiling crossbeam-epoch v0.9.1
   Compiling crossbeam-channel v0.5.0
   Compiling signal-hook v0.1.17
   Compiling backtrace v0.3.56
   Compiling url v2.2.0
   Compiling crossbeam-deque v0.8.0
   Compiling openssl-src v111.13.0+1.1.1i
   Compiling chrono v0.4.19
   Compiling libz-sys v1.1.2
   Compiling libssh2-sys v0.2.21
   Compiling libgit2-sys v0.12.18+1.1.0
   Compiling openssl-sys v0.9.60
   Compiling simplelog v0.8.0
   Compiling thiserror-impl v1.0.23
   Compiling thiserror v1.0.23
   Compiling which v4.0.2
   Compiling crossterm v0.18.2
   Compiling ron v0.6.4
   Compiling tui v0.13.0
   Compiling git2 v0.13.17
   Compiling asyncgit v0.11.0
   Compiling gitui v0.11.0
    Finished release [optimized] target(s) in 7m 14s
  Installing /home/kvogel/.cargo/bin/gitui
   Installed package `gitui v0.11.0` (executable `gitui`)
21/02/4 10:32:25 kvogel-elitebook:~/po/gitui ±(master)
❯ gitui
21/02/4 10:33:23 kvogel-elitebook:~/po/gitui ±(master)
❯
```

### Macros

[The Little Book of Rust Macros ](https://danielkeep.github.io/tlborm/book/README.html)
[So, what are hygienic macros anyway? : rust ](https://www.reddit.com/r/rust/comments/5v8r8f/so_what_are_hygienic_macros_anyway/)
[A guide on how to write hygienic Rust macros · GitHub ](https://gist.github.com/Koxiaet/8c05ebd4e0e9347eb05f265dfb7252e1)
[proc_macro::Span - Rust ](https://doc.rust-lang.org/stable/proc_macro/struct.Span.html)


[msrv rust](https://www.google.com/search?q=msrv+rust)
>Minimum Supported Rust Version (MSRV) 
[Latest stable rust as a MSRV policy · GitHub ](https://gist.github.com/alexheretic/d1e98d8433b602e57f5d0a9637927e0c)
[wg/msrv.md at master · rust-embedded/wg · GitHub ](https://github.com/rust-embedded/wg/blob/master/ops/msrv.md)
[rust/weird-exprs.rs at master · rust-lang/rust · GitHub ](https://github.com/rust-lang/rust/blob/master/src/test/ui/weird-exprs.rs#L86)

[Learn Rust by writing a simple game  Opensource.com ](https://opensource.com/article/20/12/learn-rust?sc_cid=7013a0000026LOsAAM)
[rust some](https://www.google.com/search?q=rust+some&ie=UTF-8)
[Question What is the Some keyword? : rust ](https://www.reddit.com/r/rust/comments/4ryu7a/question_what_is_the_some_keyword/)
[rust - What are Some and None?](https://stackoverflow.com/questions/24771655/what-are-some-and-none)

[Post-Mozilla Rust: The Future of the Rust Language  by Cameron Manavian  The Innovation  Medium ](https://medium.com/the-innovation/post-mozilla-rust-the-future-of-the-rust-language-61a5cfb1f615)

### Rust

[Porting Java's ConcurrentHashMap to Rust (part 1)](https://www.youtube.com/watch?list=PLqbS7AVVErFj824-6QgnK_Za1187rNfnl&v=yQFWmGaFBjk&feature=youtu.be)
[Crust of Rust: Smart Pointers and Interior Mutability](https://www.youtube.com/watch?v=8O0Nt9qY_vo)
[The Tragedy of systemd](https://www.youtube.com/watch?v=o_AIw9bGogo&t=657s)
[Rust at speed — building a fast concurrent database](https://www.youtube.com/watch?v=s19G6n0UjsM)
[crossbeam::epoch](https://www.google.com/search?q=crossbeam%3A%3Aepoch&ie=UTF-8)
[Code and Bitters ](https://codeandbitters.com/learning-rust-crossbeam-epoch/)
[Writing an OS in Rust ](https://os.phil-opp.com/)
[Hecto: Build your own text editor in Rust – Philipp Flenker](https://www.philippflenker.com/hecto/)
>Agile Native · Digital Transformation Agent

[Awesome Yew  Curated list of awesome lists  Project-Awesome.org ](https://project-awesome.org/jetli/awesome-yew)
>[Yew](https://github.com/yewstack/yew) is a modern Rust framework inspired by Elm and React for creating multi-threaded frontend apps with WebAssembly.

roguelike (rustlike) - Squall?
rust tech test

There are three main concepts with Rust:
* Ownership (only one variable "owns" the data at one time, and the owner is in charge of deallocating)
* Borrowing (you can borrow a reference to an owned variable)
* Lifetimes (all data keeps track of when it will be destroyed)


[I Am Devloper Tweeted: installing Rust for the first time - chrisjbird@gmail.com - Gmail ](https://mail.google.com/mail/u/0/#inbox/FMfcgxwJXftSshQMgfpRrcCcxVbSBRxL)
[I Am Devloper on Twitter: "installing Rust for the first time https://t.co/IZfmuiUnnu" / Twitter ](https://twitter.com/iamdevloper/status/1301846362762420224?cn=ZmxleGlibGVfcmVjcw%3D%3D&refsrc=email)
[tail call optimization](https://www.google.com/search?q=tail+call+optimization&ie=UTF-8)
[algorithm - What is tail call optimization?](https://stackoverflow.com/questions/310974/what-is-tail-call-optimization)
[Tail call - Wikipedia ](https://en.wikipedia.org/wiki/Tail_call)
[borrow checker](https://www.google.com/search?q=borrow+checker&ie=UTF-8)
[References and Borrowing ](https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html)
[Understanding the Rust borrow checker - LogRocket Blog ](https://blog.logrocket.com/introducing-the-rust-borrow-checker/)

>Oh don’t piss off the “ex-C++ senior devs who are now trying to learn Rust and Go”

[Thought you loved Python? Wait until you meet Rust  by Rhea Moutafis  Jul, 2020  Towards Data Science ](https://towardsdatascience.com/thought-you-loved-python-wait-until-you-meet-rust-64a06d976ce)
[What is Ownership? - The Rust Programming Language ](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
[Rust Ownership by Example  Depth-First ](https://depth-first.com/articles/2020/01/27/rust-ownership-by-example/)
[Rust-Powered Command-Line Utilities to Increase Your Productivity  by Shinichi Okada  Jun, 2020  Towards Data Science ](https://towardsdatascience.com/rust-powered-command-line-utilities-to-increase-your-productivity-eea03a4cf83a)
[Code of conduct - Rust Programming Language ](https://www.rust-lang.org/policies/code-of-conduct)


Oxford Rust Meetup Group
[Online inaugral Oxford Rust meetup](https://www.meetup.com/Oxford-Rust-Meetup-Group/events/272424077/)

[What is Rust and why is it so popular?Blog ](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/)
[You Want to Learn Rust but You Don’t Know Where to Start  by Shinichi Okada  Jun, 2020](https://towardsdatascience.com/you-want-to-learn-rust-but-you-dont-know-where-to-start-fc826402d5ba)

[RUST markdown in comments](https://www.google.com/search?q=RUST+markdown+in+comments&ie=UTF-8)
[Is there an actual reference for the sort of markdown used by the Rust doc comments? : rust ](https://www.reddit.com/r/rust/comments/6g6922/is_there_an_actual_reference_for_the_sort_of/)
[rustdoc uses hoedown](https://www.google.com/search?q=rustdoc+uses+hoedown&ie=UTF-8)
[hoedown 1.1.9 - Docs.rs ](https://docs.rs/crate/hoedown/1.1.9/builds/3808)
[hoedown/hoedown: Standards compliant, fast, secure markdown processing library in C ](https://github.com/hoedown/hoedown)
[Is there any documentation style guide for comments? : rust ](https://www.reddit.com/r/rust/comments/ahb50s/is_there_any_documentation_style_guide_for/)
[Comments and Documenting the code  Learning Rust ](https://learning-rust.github.io/docs/a5.comments_and_documenting_the_code.html)
[commonmark](https://www.google.com/search?q=commonmark&ie=UTF-8)
[Markdown - Wikipedia ](https://en.wikipedia.org/wiki/Markdown)

[Porous Absorber - A story of Rust and WASM  Meetup ](https://www.meetup.com/Oxford-Rust-Meetup-Group/events/273451099)
[ChrisWhealy/porous_absorber: Calculates the acoustic absorption curve of various porous absorber systems ](https://github.com/ChrisWhealy/porous_absorber)
