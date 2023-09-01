
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



[Rust for Non-Systems Programmers ](https://becca.ooo/rustconf/2020/#/)
[Rust for Non-Systems Programmers Rebecca Turner](https://www.google.com/search?q=Rust+for+Non-Systems+Programmers+Rebecca+Turner&ie=UTF-8)
[RustConf 2020 - Rust for Non-Systems Programmers by Rebecca Turner](https://www.youtube.com/watch?v=BBvcK_nXUEg)

things Rust can make "strikingly easy"

cmd arg parsing generated from type definition
automatic typo correction, autocompletion, generate man pages at compile time

complex error report, deserialize json

fancy test diffs with one line
and a lot more

solve a problem in a Rusty way

I have ADHD - working memory
powerful compiler, linter, tests
work with the compiler

rustdoc api docs mdBook
rls language server, rust-analyzer (commumity project)
cargo package manager, build system

docs.rs

thread_rng
RngCore
uniform documentation style/interface format

[Rust, WebAssembly, and the future of Serverless by Steve Klabnik](https://www.youtube.com/watch?v=CMB6AlE1QuI)
[wasm wasi](https://www.google.com/search?q=wasm+wasi&ie=UTF-8)
[WASI  ](https://wasi.dev/)

>Rust: A language empowering everyone to build reliable and efficient software
iterated several times



[C++ && Rust : "Access All Arenas" - James Munns](https://www.youtube.com/watch?v=HiWkMFE8uRE)
[We need a safer systems programming language – Microsoft Security Response Center ](https://msrc-blog.microsoft.com/2019/07/18/we-need-a-safer-systems-programming-language/)

Graydon Hoare, Mozilla and community
designed safety, concurrency and speed - critical piece of the Servo project to develop a parallel browser engine

zero-cost abstractions
safety is not optional by default, e.g. compiler warnings - opt-out, not opt-in (as in C++)


>Heater 4 months ago A great presentation. Thanks. With regard the performance of Rust, in my first year of Rust have been able to match the speed of C/C++ in all the little programs of mine that I reimplemented in Rust for exactly this comparison. Most of those C/C++ programs were performance leaders in various coding challenges I have been involved in. BUT: The performance of Rust can be very sensitive to the way you write your Rust, even for the same algorithm. For example using good old fashioned C style array indexing vs Rust style iterators. It's not always clear which will be better at the outset. As usual, measure everything.

>piggy66 1 month ago anyone else find rust programmers so smug and condescending? i cant stand them


[What Rust is it? ](https://www.whatrustisit.com/)
[Why scientists are turning to Rust ](https://www.nature.com/articles/d41586-020-03382-2)
[The Rust Programming Language - The Rust Programming Language ](https://doc.rust-lang.org/book/)
[Table of Contents - Rust Cookbook ](https://rust-lang-nursery.github.io/rust-cookbook/intro.html)


[Parsing and Displaying - Rust Cookbook ](https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html#convert-date-to-unix-timestamp-and-vice-versa)
[Duration and Calculation - Rust Cookbook ](https://rust-lang-nursery.github.io/rust-cookbook/datetime/duration.html#measure-the-elapsed-time-between-two-code-sections)


[unreal engine rust](https://www.google.com/search?q=unreal+engine+rust&ie=UTF-8)
[unity rust lang](https://www.google.com/search?q=unity+rust+lang&ie=UTF-8)
[Call into Rust from C# and Unity. Rust is a safe and fast low-level…  by Emanuele Manzione  Medium ](https://medium.com/@mhlab/call-into-rust-from-c-and-unity-35065f9f2c7d)
[interop](https://www.google.com/search?q=interop&ie=UTF-8)
[Interoperability - C# Programming Guide  Microsoft Docs ](https://docs.microsoft.com/en-us/dotnet/csharp/programming-guide/interop/)
[Rust Once, Run Everywhere  Rust Blog ](https://blog.rust-lang.org/2015/04/24/Rust-Once-Run-Everywhere.html)
[Unsafe Rust - The Rust Programming Language ](https://doc.rust-lang.org/1.30.0/book/2018-edition/ch19-01-unsafe-rust.html)
[Rust(lang) in Unity3D. How to use Unity’s Native Plugin…  by Jim Fleming  Jim Fleming  Medium ](https://medium.com/jim-fleming/rust-lang-in-unity3d-eeaeb47f3a77)
[Rust Once, Run Everywhere  Rust Blog ](https://blog.rust-lang.org/2015/04/24/Rust-Once-Run-Everywhere.html)
[Rust(lang) in Unity3D. How to use Unity’s Native Plugin…  by Jim Fleming  Jim Fleming  Medium ](https://medium.com/jim-fleming/rust-lang-in-unity3d-eeaeb47f3a77)
[Rust + Unity3D? : rust_gamedev ](https://www.reddit.com/r/rust_gamedev/comments/9zmmwx/rust_unity3d/)
[Rust(lang) in Unity3D : Unity3D ](https://www.reddit.com/r/Unity3D/comments/37r294/rustlang_in_unity3d/)
[How Amethyst compares with C++ giants Unity/Unreal. : rust_gamedev ](https://www.reddit.com/r/rust_gamedev/comments/cb0u9l/how_amethyst_compares_with_c_giants_unityunreal/)
[Would the Rust programming language be a good match for game development with Unity? - Unity Forum ](https://forum.unity.com/threads/would-the-rust-programming-language-be-a-good-match-for-game-development-with-unity.477218/)
[Pinterest ](https://www.pinterest.co.uk/pin/847732329845586204/?autologin=true)
[Why Rust is the Future of Game Development - TheFuntastic ](https://thefuntastic.com/blog/why-rust-is-the-future-game-dev)
[Graydon Hoare](https://www.google.com/search?q=Graydon+Hoare&ie=UTF-8)
[NVMe](https://www.google.com/search?q=NVMe&ie=UTF-8)
[What is NVMe and Why is it Important? A Technical Guide ](https://blog.westerndigital.com/nvme-important-data-driven-businesses/)
[ThinkPad W530 Mobile Workstation  Portable Laptop  Lenovo US ](https://www.lenovo.com/us/en/laptops/thinkpad/w-series/w530/)
[Lenovo ThinkPad X61 Series Specs - CNET ](https://www.cnet.com/products/lenovo-thinkpad-x61-series/)
[graydon (Graydon Hoare) ](https://github.com/graydon)
[Graydon Hoare (@graydon_pub) / Twitter ](https://twitter.com/graydon_pub?lang=en)
["READ THE ROOM" - Twitter Search / Twitter ](https://twitter.com/search?q=%22READ%20THE%20ROOM%22&src=trend_click&vertical=trends)
[Churchill - Twitter Search / Twitter ](https://twitter.com/search?q=Churchill&src=trend_click&vertical=trends)
[Rust’s original creator, Graydon Hoare on the current state of system programming and safety  Packt Hub ](https://hub.packtpub.com/rusts-original-creator-graydon-hoare-on-the-current-state-of-system-programming-and-safety/)
[Graydon Hoare](https://www.google.com/search?q=Graydon+Hoare&tbm=isch&source=iu&ictx=1&fir=kJ4N4iBOLEmQeM%252CXXX9OSvY5X9v6M%252C_&vet=1&usg=AI4_-kTByMhGHZsWfG3-N9E7oq_RNEBYkA&biw=1366&bih=695#imgrc=kJ4N4iBOLEmQeM)
[I wonder, why Graydon Hoare, the author of Rust, stopped contributing into it and switched to Swift? : rust ](https://www.reddit.com/r/rust/comments/7qels2/i_wonder_why_graydon_hoare_the_author_of_rust/)
[graydon2  "What next?" ](https://graydon2.dreamwidth.org/253769.html)
[C++ && Rust : "Access All Arenas" - James Munns](https://www.youtube.com/watch?v=HiWkMFE8uRE)
[We need a safer systems programming language – Microsoft Security Response Center ](https://msrc-blog.microsoft.com/2019/07/18/we-need-a-safer-systems-programming-language/)
[Chrome: 70% of all security bugs are memory safety issues  ZDNet ](https://www.zdnet.com/article/chrome-70-of-all-security-bugs-are-memory-safety-issues/)



[Entity Component System Rust](https://www.google.com/search?q=Entity+Component+System+Rust&biw=1280&bih=616)
[Entities and Components - Roguelike Tutorial - In Rust ](https://bfnightly.bracketproductions.com/rustbook/chapter_2.html)
[ECS  Are we game yet? ](https://arewegameyet.rs/ecosystem/ecs/)
[Are we game yet? ](https://arewegameyet.rs/#about)
[Are we web yet? Yes, and it's freaking fast! ](https://www.arewewebyet.org/)
[WebAssembly » AWWY? ](https://www.arewewebyet.org/topics/webassembly/)
[FRP signals](https://www.google.com/search?q=FRP+signals&ie=UTF-8)
[A Farewell to FRP ](https://elm-lang.org/news/farewell-to-frp)
[Functional reactive programming - Wikipedia ](https://en.wikipedia.org/wiki/Functional_reactive_programming)
[Functional Reactive Programming (FRP)  by Navdeep Singh  Medium ](https://medium.com/@navdeepsingh_2336/functional-reactive-programming-frp-2826177b1e27)
[Are we learning yet? ](https://www.arewelearningyet.com/)


```
2022-07-14 20:11:44 kvogel@kvogel-surface-ubuntu:~
❯ cargo install du-dust
    Updating crates.io index
  Downloaded du-dust v0.8.1
  Downloaded 1 crate (89.1 KB) in 0.39s
  Installing du-dust v0.8.1
  Downloaded crossbeam-channel v0.5.5
  Downloaded crossbeam-utils v0.8.10
  Downloaded either v1.7.0
  Downloaded hashbrown v0.12.3
  Downloaded lscolors v0.7.1
  Downloaded indexmap v1.9.1
  Downloaded libc v0.2.126
  Downloaded clap_lex v0.2.4
  Downloaded crossbeam-epoch v0.9.9
  Downloaded thousands v0.2.0
  Downloaded strsim v0.10.0
  Downloaded termcolor v1.1.3
  Downloaded rayon v1.5.3
  Downloaded textwrap v0.15.0
  Downloaded stfu8 v0.2.5
  Downloaded regex-syntax v0.6.27
  Downloaded regex v1.6.0
  Downloaded clap v3.2.12
  Downloaded atty v0.2.14
  Downloaded rayon-core v1.9.3
  Downloaded memchr v2.5.0
  Downloaded os_str_bytes v6.2.0
  Downloaded once_cell v1.13.0
  Downloaded 23 crates (2.2 MB) in 1.33s
   Compiling autocfg v1.1.0
   Compiling libc v0.2.126
   Compiling once_cell v1.13.0
   Compiling crossbeam-utils v0.8.10
   Compiling cfg-if v1.0.0
   Compiling memchr v2.5.0
   Compiling scopeguard v1.1.0
   Compiling rayon-core v1.9.3
   Compiling regex-syntax v0.6.27
   Compiling hashbrown v0.12.3
   Compiling os_str_bytes v6.2.0
   Compiling bitflags v1.3.2
   Compiling textwrap v0.15.0
   Compiling strsim v0.10.0
   Compiling either v1.7.0
   Compiling termcolor v1.1.3
   Compiling lazy_static v1.4.0
   Compiling ansi_term v0.12.1
   Compiling unicode-width v0.1.9
   Compiling thousands v0.2.0
   Compiling clap_lex v0.2.4
   Compiling memoffset v0.6.5
   Compiling crossbeam-epoch v0.9.9
   Compiling indexmap v1.9.1
   Compiling rayon v1.5.3
   Compiling lscolors v0.7.1
   Compiling crossbeam-channel v0.5.5
   Compiling aho-corasick v0.7.18
   Compiling num_cpus v1.13.1
   Compiling atty v0.2.14
   Compiling terminal_size v0.1.17
   Compiling crossbeam-deque v0.8.1
   Compiling clap v3.2.12
   Compiling regex v1.6.0
   Compiling stfu8 v0.2.5
   Compiling du-dust v0.8.1
    Finished release [optimized] target(s) in 5m 19s
  Installing /home/kvogel/.cargo/bin/dust
   Installed package `du-dust v0.8.1` (executable `dust`)
2022-07-17 16:04:26 kvogel@kvogel-surface-ubuntu:~
❯ dust
Did not have permissions for all directories
 2.1G   ┌── snap                  │                                                                                                                                       █████████ │   6%
 1.9G   │       ┌── Bitwig        │                                                                                                                                      ░▓████████ │   5%
 1.9G   │     ┌─┴ multi-samples   │                                                                                                                                      ░▓████████ │   5%
 2.2G   │   ┌─┴ 1.0               │                                                                                                                                      ░█████████ │   6%
 2.2G   │ ┌─┴ installed-packages  │                                                                                                                                      ░█████████ │   6%
 2.2G   ├─┴ .BitwigStudio         │                                                                                                                                      ██████████ │   6%
 2.2G   │ ┌── installs            │                                                                                                                                      ██████████ │   6%
 2.3G   ├─┴ .asdf                 │                                                                                                                                      ██████████ │   7%
 2.5G   │ ┌── share               │                                                                                                                                     ███████████ │   7%
 2.5G   ├─┴ .local                │                                                                                                                                     ███████████ │   7%
 4.6G   ├── .config               │                                                                                                                             ███████████████████ │  13%
 2.0G   │           ┌── @FileStore│                                                                                                             ░░░░░░▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓█████████ │   6%
 2.0G   │         ┌─┴ hicss       │                                                                                                             ░░░░░░▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓█████████ │   6%
 2.1G   │       ┌─┴ files         │                                                                                                             ░░░░░░▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓█████████ │   6%
 2.3G   │       ├── repos         │                                                                                                             ░░░░░░▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓██████████ │   6%
 4.7G   │     ┌─┴ uhs             │                                                                                                             ░░░░░░▒▒▒▒▒▒▒▒▓████████████████████ │  13%
 4.9G   │   ┌─┴ work              │                                                                                                             ░░░░░░▒▒▒▒▒▒▒▒█████████████████████ │  14%
 6.9G   │ ┌─┴ general             │                                                                                                             ░░░░░░█████████████████████████████ │  20%
 8.3G   ├─┴ projects              │                                                                                                             ███████████████████████████████████ │  24%
  34G ┌─┴ .                       │████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████ │ 100%
```

[Is Rust the perfect programming language? | by Waracle | Medium ](https://medium.com/@WaracleUK/is-rust-the-perfect-programming-language-124c51c75067)


Carbon?? Rust, Go? Aargh!
Forget [Google Carbon](/dev/lang/carbon/carbon.md)
>If you can use Rust, ignore Carbon.

rust
[Rust Day on Google Open Source Live, Thu, Sep 1, 2022, 9:00 AM | Meetup ](https://www.meetup.com/google-open-source/events/287435626/?read_xtd=gatlbWFpbF9jbGlja9oAJGUzYmNhMmNmLWY3MjUtNDgxYi1hNjZmLWQxM2YxODUxMjhiNw)
[Home - Rust Day on Google Open Source Live ](https://opensourcelive.withgoogle.com/events/rust-day-2022)
https://www.youtube.com/GoogleOpenSource

[Rust ❤️ C++ ](https://cxx.rs/)
[Rustaceans ](https://rustaceans.org/)
rust pong, astro blaster, day4night
[google/autocxx: Tool for safe ergonomic Rust/C++ interop driven from existing C++ headers ](https://github.com/google/autocxx)
[Rust Day on Google Open Source Live, Thu, Sep 1, 2022, 9:00 AM | Meetup ](https://www.meetup.com/google-open-source/events/287435626/)
[Home - Rust Day on Google Open Source Live ](https://opensourcelive.withgoogle.com/events/rust-day-2022?utm_sourceutm_mediumutm_campaign=Rust%2BDay)

[Should we call Rust a Failed Programming Language? ](https://analyticsindiamag.com/should-we-call-rust-a-failed-programming-language/)
>Google engineer Chandler Carruth advised those using ‘Rust’ to continue using it. Carbon is for developers with large codebases in C++, which are difficult to convert into Rust.
>Rust is difficult. It has a complex syntax and a steep learning curve. It is designed to uniquely solve some very challenging problems in programming. However, as a beginner, using Cuda or MPI on Rust is not very simple compared to the other options like Swift and Go. Moreover, it is slow. Rust is a snail compared to other languages. Even for small projects, the compile times are painfully long, and runtime measurements show that Rust is less efficient than the C programs.
[The Rust Programming Language ](https://www.reddit.com/r/rust/)
[An almost religious case for Rust | by Alexander Sidorov | Sep, 2022 | Medium ](https://medium.com/@siberianguy/an-almost-religious-case-for-rust-e4c4764acd8d)
[Why is Rust the Most Loved Programming Language? ](https://matklad.github.io/2020/02/14/why-rust-is-loved.html)
[For Complex Applications, Rust is as Productive as Kotlin ](https://ferrous-systems.com/blog/rust-as-productive-as-kotlin/)

astroblaster... bevy 2d tutorial

[Rust for Old People. How life after C, C++, and Python feels… | by Doug Foo | CodeX | Medium ](https://medium.com/codex/rust-for-old-people-516fc72b2934)
[algorithm - What is tail call optimization?](https://stackoverflow.com/questions/310974/what-is-tail-call-optimization)
[Tail call](https://en.wikipedia.org/wiki/Tail_call)
[27. Tail call optimization ](https://exploringjs.com/es6/ch_tail-calls.html)

[Cool Features of Rust (for Old People) | by Doug Foo | CodeX | Medium ](https://medium.com/codex/cool-features-of-rust-for-old-people-10ba6a0991b6)
>In Rust there is no Class-Object system, and enums are one of the 4 major building blocks in the language. Enums can also hold values and host functions (hmm sounds like a class..):
```rust
enum Number {
    Rational(i32,i32),
    Integer(i32),
    Float(f32)
}
impl Number {
   fn show(&self) {
       println!("call ({})", self);
   }
}
let n = Number::Integer(20);
n.show();  // calls show->fmt::Display
```

>In place of Classes are Traits (along with Enums and Structs), which are kind of like interfaces with more power.
```rust
trait Hash {
  fn hash(&self) -> u64;
}
impl Hash for Number {
  fn hash(&self) -> u64 {
    if *self { 0 } else { 1 }
    }
}
impl Hash for bool { ... }   // yea this works, crazy cool
```


["The Fatal Flaw of Ownership Semantics" : rust ](https://www.reddit.com/r/rust/comments/xc8enj/the_fatal_flaw_of_ownership_semantics/)
[media ripdrag - Drag and Drop utility to make your terminal life easier! : rust ](https://www.reddit.com/r/rust/comments/xbssv2/media_ripdrag_drag_and_drop_utility_to_make_your/)
[xselect](https://www.google.com/search?q=xselect&ie=UTF-8)
["package.exclude" field of Cargo.toml](https://www.google.com/search?q=%22package.exclude%22+field+of+Cargo.toml&ie=UTF-8)
[The Manifest Format - The Cargo Book ](https://doc.rust-lang.org/cargo/reference/manifest.html)
[The Manifest Format - The Cargo Book ](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/cargo/reference/manifest.html)
[Change Cargo include/exclude rules to gitignore patterns · Issue #4268 · rust-lang/cargo ](https://github.com/rust-lang/cargo/issues/4268)
[package.exclude](https://www.google.com/search?q=package.exclude&ie=UTF-8)
[Serverless Framework - Packaging ](https://www.serverless.com/framework/docs/providers/aws/guide/packaging)
[The Fatal Flaw of Ownership Semantics - gingerBill ](https://www.gingerbill.org/article/2020/06/21/the-ownership-semantics-flaw/)
[Unsafe Rust - The Rust Programming Language ](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)

[Why we’ve decided to use Rust-lang | by Edoardo Barp | Jul, 2022 | Medium ](https://barp-edoardo.medium.com/why-weve-decided-to-use-rust-lang-8e38bd1be78f)
>At X80 Security, we strive at bringing the 4th Generation of Cyber Security to the Enterprise to stop the coming wave of sophisticated Cyber Attacks.
→ Follow us on Linkedin
→ Reach out to us
[Why we’ve decided to use Rust-lang | by Edoardo Barp | Jul, 2022 | Medium ](https://barp-edoardo.medium.com/why-weve-decided-to-use-rust-lang-8e38bd1be78f)
[(99+) X80 Security: Overview | LinkedIn ](https://www.linkedin.com/company/x80-security/)
[X80 Security Contact form ](https://docs.google.com/forms/d/e/1FAIpQLSfuZPqM8k0g-gWqH0nd9FYd__jxTsl4ZhB6Y3hvJVTK8DlK7w/viewform)
[Simulated AI — The 4th Generation of Cyber Defences | by Simon Janin | Medium ](https://sjanin.medium.com/simulated-ai-the-4th-generation-of-cyber-defences-f47b80cfe2b2)

[Game Development with Rust | The Pragmatic Programmers ](https://medium.com/pragmatic-programmers/game-development-with-rust-31147f7b6096)

[Rust for Non-Systems Programmers ](https://becca.ooo/rustconf/2020/#/)
[RustConf 2020 - Rust for Non-Systems Programmers by Rebecca Turner](https://www.youtube.com/watch?v=BBvcK_nXUEg)

[https://www.diigo.com ](https://www.diigo.com/user/chrisjbird/?page=3&query=rustlang)
[Rust for Old People. How life after C, C++, and Python feels…  by Doug Foo  CodeX  Medium ](https://medium.com/codex/rust-for-old-people-516fc72b2934)
[Mental models for learning Rust ](https://kerkour.com/rust-mental-models)
[Rust and OpenCV. We all know why Rust is so great…  by Jonathan Österberg  Dev Genius ](https://blog.devgenius.io/rust-and-opencv-bb0467bf35ff)
[rust - What are the identifiers denoted with a single apostrophe (')?](https://stackoverflow.com/questions/22048673/what-are-the-identifiers-denoted-with-a-single-apostrophe)
[Learn Rust Programming: The Complete Developer's Guide  Zero To Mastery ](https://zerotomastery.io/courses/learn-rust/)

[rust iced vs egui](https://www.google.com/search?q=rust+iced+vs+egui&ie=UTF-8)
[egui or iced or druid? : rust ](https://www.reddit.com/r/rust/comments/th0m45/egui_or_iced_or_druid/)
>Egui has been a pleasure to work with, and Bevy as well. Unless you direly need accessibility, Egui is the best gui library for Rust right now.
>dioxus is worth a shot. It’s extremely pleasant to work with it, since it’s basically react but in rust.
>SoSmartFlow OP But I don't know react
>Mag_SG 8 mo. ago Then you should read their book. It explains the basic concepts very well, with lots of code examples. https://dioxuslabs.com/guide/index.html
>Difficult_Owl493 WOW!!!!This Really Great!!! Egui is easy to use, but the layout and style sucks. React NO1!

[State of Rust GUI in 2021? : rust ](https://www.reddit.com/r/rust/comments/rgyqov/comment/honc023/)
[Iced vs Egui? : learnrust ](https://www.reddit.com/r/learnrust/comments/nehewg/iced_vs_egui/)
[Egui vs iced in regards to game engine integration - The Rust Programming Language Forum ](https://users.rust-lang.org/t/egui-vs-iced-in-regards-to-game-engine-integration/74569)
[Recommended Rust GUI Libraries : rust ](https://www.reddit.com/r/rust/comments/q14xu0/recommended_rust_gui_libraries/)



### Rust colour

[Rusty colour themes : rust ](https://www.reddit.com/r/rust/comments/dsbdmg/rusty_colour_themes/)
From footer of [Rust Foundation - Logo Policy and Media Guide ](https://foundation.rust-lang.org/policies/logo-policy-and-media-guide/): `#f46624`

[Rust: Result Type Explained. Understanding the Rust Result type for…  by Pascal Zwikirsch  Dec, 2022  Level Up Coding ](https://levelup.gitconnected.com/rust-result-type-explained-c0162b363a5f)


### Rust Nation UK 2023 videos

[Rust Nation UK 2023](https://www.youtube.com/playlist?list=PL1AoGvxomykTuOMzY5KrI4WiPCsIlYnAM)

### RustConf 2023 Fiasco

[Carter Anderson on Twitter: "I use Rust because there is literally nothing else on the market like it. The character of other "similar" languages (Zig, Go, C++, Jai, etc) is completely different. The developer community is the best I have ever been in, despite the hiccups here and there." / Twitter ](https://twitter.com/cart_cart/status/1664064410086780931)
[The RustConf Keynote Fiasco, explained ](https://fasterthanli.me/articles/the-rustconf-keynote-fiasco-explained#a-bit-of-context)
[Rust: The wrong people are resigning ](https://fasterthanli.me/articles/rust-the-wrong-people-are-resigning)
[mod team resignation by BurntSushi · Pull Request #671 · rust-lang/team ](https://github.com/rust-lang/team/pull/671)
[I Am No Longer Speaking at RustConf 2023  The Pasture ](https://thephd.dev/i-am-no-longer-speaking-at-rustconf-2023)
[A Mirror for Rust: Compile-Time Reflection Report - Shepherd's Oasis ](https://soasis.org/posts/a-mirror-for-rust-a-plan-for-generic-compile-time-introspection-in-rust/)
[Rust](https://www.youtube.com/@RustVideos)
[Why I left Rust ](https://www.jntrnr.com/why-i-left-rust/)
[RustConf](https://www.google.com/search?q=RustConf&ie=UTF-8)
[On the RustConf keynote  Rust Blog ](https://blog.rust-lang.org/2023/05/29/RustConf.html)
[A post on the RustConf keynote fiasco LWN.net ](https://lwn.net/Articles/933486/)
[Stepping Down From RustConf and Rust Project Controversy](https://www.youtube.com/watch?v=TB0cXGvuw9A)
[I Am No Longer Speaking at RustConf 2023  The Pasture ](https://thephd.dev/i-am-no-longer-speaking-at-rustconf-2023)
[⭐ About  The Pasture ](https://thephd.dev/about/)
[⭐ JeanHeyd Meneide  LinkedIn ](https://www.linkedin.com/in/thephd/)
[Björkus "Fix It Or Die Trying" Dorkus (@__phantomderp) / Twitter ](https://twitter.com/__phantomderp/)
[derpconf 2020 - Derp - The... Community?](https://www.youtube.com/watch?v=vaLKm9FE8oo)
[ThePhD (The Phantom Derpstorm) ](https://github.com/ThePhD)
[JeanHeyd Meneide](https://www.google.com/search?q=JeanHeyd+Meneide&ie=UTF-8)
[Interview with JeanHeyd Meneide, Project Editor the ISO C standard](https://www.youtube.com/watch?v=A1NcqcURfWc)

---

[Rust and TUI: A Romantic Affair. Rust: A brief history  by Tapas Das  Medium ](https://tdtapas.medium.com/rust-and-tui-a-romantic-affair-a96925da32ac)
