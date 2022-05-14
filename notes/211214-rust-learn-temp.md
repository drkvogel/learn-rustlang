



>by default variables are immutable. This is one of many nudges Rust gives you to write your code in a way that takes advantage of the safety and easy concurrency that Rust offers.

>hadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

>The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. 

>how do you know which type of integer to use? If you’re unsure, Rust’s defaults are generally good choices, and integer types default to i32: this type is generally the fastest, even on 64-bit systems. The primary situation in which you’d use isize or usize is when indexing some sort of collection.

>Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.

>Rust has two primitive compound types: tuples and arrays.

>Tuples have a fixed length: once declared, they cannot grow or shrink in size.
```rs
let tup: (i32, f64, u8) = (500, 6.4, 1);

let (x, y, z) = tup; // pattern matching/destructuring

 let five_hundred = x.0;
```

>Unlike a tuple, every element of an array must have the same type. Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.

>Arrays are useful when you want your data allocated on the stack rather than the heap (we will discuss the stack and the heap more in Chapter 4) or when you want to ensure you always have a fixed number of elements. An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. If you’re unsure whether to use an array or a vector, you should probably use a vector.

>When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than the array length. If the index is greater than or equal to the array length, Rust will panic.

>This is the first example of Rust’s safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing.

>Rust doesn’t care where you define your functions, only that they’re defined somewhere.

>Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.

>Functions can return values to the code that calls them. We don’t name return values, but we do declare their type after an arrow (->).

>You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly. 


gtk-rs
druid
iced
tauri

>Rust provides a Foreign Function Interface (FFI) to C libraries. Foreign functions must be declared inside an extern block annotated with a #[link] attribute containing the name of the foreign library.

[Rust for Non-Systems Programmers ](https://becca.ooo/rustconf/2020/#/)
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

`thread_rng`
`RngCore`
uniform documentation style/interface format

[Rust, WebAssembly, and the future of Serverless by Steve Klabnik](https://www.youtube.com/watch?v=CMB6AlE1QuI)
[wasm wasi](https://www.google.com/search?q=wasm+wasi&ie=UTF-8)
[WASI  ](https://wasi.dev/)

Rust: A language empowering everyone to build reliable and efficient software
iterated several times

### Game engines

Game yet
Amethyst
Bevy ("Amethyst 2.0"?)
RG3D - looks amazing!
Godot - FOSS Unity alternative - C++, has Rust bindings?


[godot vs amethyst vs rg3d vs bevy](https://www.google.com/search?q=godot+vs+amethyst+vs+rg3d+vs+bevy&ie=UTF-8)
[Godot](https://www.google.com/search?q=Godot&ie=UTF-8)
[A refreshingly simple data-driven game engine built in Rust  RustRepo ](https://rustrepo.com/repo/bevyengine-bevy-rust-game-development)
[Ggez vs Bevy vs Piston vs Amethyst : rust_gamedev ](https://www.reddit.com/r/rust_gamedev/comments/nwoh2k/ggez_vs_bevy_vs_piston_vs_amethyst/)
[Help choosing a game engine. : rust_gamedev ](https://www.reddit.com/r/rust_gamedev/comments/k0ol2z/help_choosing_a_game_engine/)
[ECS from Specs to Legion](https://www.google.com/search?q=ECS+from+Specs+to+Legion&ie=UTF-8)
[Specs and Legion, two very different approaches to ECS ](https://csherratt.github.io/blog/posts/specs-and-legion/)
[Specs and Legion, two very different approaches to ECS : rust ](https://www.reddit.com/r/rust/comments/fsczky/specs_and_legion_two_very_different_approaches_to/)
[amethyst/legion: High performance Rust ECS library ](https://github.com/amethyst/legion)
[dasifefe/rust-game-development-frameworks: List of curated frameworks by the **Game Development in Rust** community. ](https://github.com/dasifefe/rust-game-development-frameworks)

[Introduction - Roguelike Tutorial - In Rust ](https://bfnightly.bracketproductions.com/rustbook/)

[godot-rust/godot-rust: Rust bindings for GDNative ](https://github.com/godot-rust/godot-rust)
[Godot Rust · Hagsteel ](https://hagsteel.com/posts/godot-rust/)
[FyroxEngine/Fyrox: 3D and 2D game engine written in Rust ](https://github.com/FyroxEngine/Fyrox)
[bevy vs amethyst](https://www.google.com/search?q=bevy+vs+amethyst&ie=UTF-8)
[Bevy vs Unity](https://www.google.com/search?q=Bevy+vs+Unity&sa=X&ved=2ahUKEwiRzfzY3Lb1AhWG73MBHc75DIgQ1QJ6BAgZEAE)
[Rg3d vs Bevy](https://www.google.com/search?q=Rg3d+vs+Bevy&sa=X&ved=2ahUKEwiRzfzY3Lb1AhWG73MBHc75DIgQ1QJ6BAgWEAE)
[Are we game yet? ](https://arewegameyet.rs/)
[mrDIMAS/rusty-shooter: suspended 3d shooter written in Rust using rg3d ](https://github.com/mrDIMAS/rusty-shooter)
[amethyst/amethyst: Data-oriented and data-driven game engine written in Rust ](https://github.com/amethyst/amethyst)
[GitHub Actions CI, Take 3 by CleanCut · Pull Request #2382 · amethyst/amethyst ](https://github.com/amethyst/amethyst/pull/2382)


bevy vs godot? sounds like bevy will catch up soon if it is behind godot, and I want to use pure Rust, not C++

bevy.md
learn-rustlang.git clone?

### Bevy

[Introduction - Unofficial Bevy Cheat Book ](https://bevy-cheatbook.github.io/)
[Bevy - Setup ](https://bevyengine.org/learn/book/getting-started/setup/)
>I predict that Bevy will surpass Amethyst in terms of features very soon and will be a proper engine usable for a full game in a few months (although people (incl. myself) are working on "serious" game projects in Bevy already).

[Bevy Engine – Addressing the elephant in the room - Community - Amethyst Forums ](https://community.amethyst.rs/t/bevy-engine-addressing-the-elephant-in-the-room/1645)
>Bevy is essentially a Amethyst Engine 2.0. 
>Bevy ECS 58 is a fork of hecs 95 which in turn is a trimmed down version of Legion. Bevy’s additions brings its ECS back full circle to something quite similar to the newest Legion v0.3.

>Bevy is pursuing a Godot-style approach to its Editor UI, which is to develop its in-game UI and Editor UI as one unified project. 

>What's a BEVY? A bevy is a group of birds!

[Bevy - Introduction ](https://bevyengine.org/learn/book/introduction/)
[Bevy - A data-driven game engine built in Rust ](https://bevyengine.org/)

[A refreshingly simple data-driven game engine built in Rust  RustRepo ](https://rustrepo.com/repo/bevyengine-bevy-rust-game-development)
[Bevy - Introduction ](https://bevyengine.org/learn/book/introduction/)
[bevyengine/bevy-assets: A collection of Bevy assets, plugins, learning resources, and apps made by the community ](https://github.com/bevyengine/bevy-assets)
[Discord ](https://discord.com/channels/691052431525675048/691052431974465548)
[Bevy Engine ](https://www.reddit.com/r/bevy/)
[Newest 'bevy' Questions](https://stackoverflow.com/questions/tagged/bevy)

[Bevy - Assets ](https://bevyengine.org/assets/#learning)
[github.com ](https://github.com/jcornaz/heron)
[github.com ](https://github.com/james7132/bevy-steamworks)


[bevy - crates.io: Rust Package Registry ](https://crates.io/crates/bevy)
[Bevy - Introduction ](https://bevyengine.org/learn/book/introduction/)
[Bevy - A data-driven game engine built in Rust ](https://bevyengine.org/)
[A bevy is a group of birds!](https://www.google.com/search?q=A+bevy+is+a+group+of+birds!&ie=UTF-8)
[bevy/examples at latest · bevyengine/bevy ](https://github.com/bevyengine/bevy/tree/latest/examples)
[Bevy - Assets ](https://bevyengine.org/assets/#learning)
[jcornaz/heron: An ergonomic physics API for bevy games. ](https://github.com/jcornaz/heron)
[james7132/bevy-steamworks: A Bevy plugin for integrating with the Steamworks SDK ](https://github.com/james7132/bevy-steamworks)
[Steamworks SDK (Steamworks Documentation) ](https://partner.steamgames.com/doc/sdk)
[ramirezmike/not_snake_game: A snake-inspired game made in Rust using the Bevy game engine. ](https://github.com/ramirezmike/not_snake_game)
[Not Snake by Michael Ramirez ](https://ramirezmike2.itch.io/not-snake)
[github.com ](https://github.com/TheRealTeamFReSh/MurderUserDungeon)
[github.com ](https://github.com/bonsairobo/bevy_rider)
[github.com ](https://github.com/rparrett/taipo)
[github.com ](https://github.com/notation-fun/notation)
[github.com ](https://github.com/bonsairobo/bevy_rider)
[gitlab.com ](https://gitlab.com/BottledByte/per-spatium)
[github.com ](https://github.com/SuperiorJT/bevy_pong)
[github.com ](https://github.com/WhoisDavid/bevy-nbody)
[github.com ](https://github.com/insrcd/labrynth-game)
[github.com ](https://github.com/cryscan/summer-jam)
[github.com ](https://github.com/Bobox214/Kataster)
[bevy/examples at latest · bevyengine/bevy ](https://github.com/bevyengine/bevy/tree/latest/examples)
[lmao ](https://caballerocoll.com/blog/bevy-rhythm-game/)
[Creating a Snake Clone in Rust, with Bevy :: A blog — Just a blog ](https://mbuffett.com/posts/bevy-snake-tutorial/)
[Gravity and Physics in the Bevy Game Engine // Spencer B. ](https://sburris.xyz/posts/bevy-gravity/)

[Amethyst or Bevy? : rust ](https://www.reddit.com/r/rust/comments/k8kv3z/amethyst_or_bevy/)
[Bevy Engine – Addressing the elephant in the room - Community - Amethyst Forums ](https://community.amethyst.rs/t/bevy-engine-addressing-the-elephant-in-the-room/1645)
[Bevy - ECS ](https://bevyengine.org/learn/book/getting-started/ecs/)
[Introduction - Roguelike Tutorial - In Rust ](https://bfnightly.bracketproductions.com/rustbook/)
[Bevy Editor · Issue #85 · bevyengine/bevy ](https://github.com/bevyengine/bevy/issues/85)
[Bevy Engine – Addressing the elephant in the room - Community - Amethyst Forums ](https://community.amethyst.rs/t/bevy-engine-addressing-the-elephant-in-the-room/1645)
[Feature parity with Amethyst · Discussion #1345 · bevyengine/bevy ](https://github.com/bevyengine/bevy/discussions/1345)
[Bevy - Faq ](https://bevyengine.org/learn/book/faq/)
>Why create Bevy when INSERT-GAME-ENGINE-HERE exists? There are plenty of fantastic engines out there ... why build another one? Especially when there are already so many in the Rust ecosystem? First a bit about me: I decided to build Bevy after years of contributing code to other engines (ex: Godot). I spent over four years building a game in Godot and I also have experience with Unity, Unreal, and a number of other frameworks like SDL and Three.js. I have built multiple custom engines in the past using Rust, Go, HTML5, and Java. I have also used and/or closely followed most of the current players in the Rust gamedev ecosystem. I recently quit my job as Senior Software Engineer at Microsoft and my experience there deeply affected my opinions of software and what it should be.

[cart](https://www.youtube.com/c/cartdev) (Bevy BDFL)


### Macroquad?

[Macroquad](https://www.google.com/search?q=Macroquad&ie=UTF-8)
[not-fl3/macroquad: Cross-platform game engine in Rust. ](https://github.com/not-fl3/macroquad)
[Macroquad ](https://macroquad.rs/)
[macroquad - Rust ](https://docs.rs/macroquad/latest/macroquad/)
[Rust 🦀 Game Programming 💥 MacroQuad / MiniQuad 💥 Implement a game - Part -1](https://www.youtube.com/watch?v=xycqxQ728ZY)


---
bevy vs godot? sounds like bevy will catch up soon if it is behind godot, and I want to use pure Rust, not C++

/home/kvogel/projects/general/dev/games/day4night/notes/day4night.md




[Making WebAssembly better for Rust & for all languages – Mozilla Hacks – the Web developer blog ](https://hacks.mozilla.org/2018/03/making-webassembly-better-for-rust-for-all-languages/)
[Rust](https://www.google.co.uk/search?q=Rust&ie=UTF-8)
[rust game](https://www.google.co.uk/search?q=rust+game&sa=X&ved=0ahUKEwjG87bFt8_bAhVBC8AKHWWWDsYQ1QII6gEoAA)
[rust language](https://www.google.co.uk/search?q=rust+language&sa=X&ved=0ahUKEwjG87bFt8_bAhVBC8AKHWWWDsYQ1QII7AEoAg)
[Why Rust is the Most Loved Language by Developers – Mozilla Tech – Medium ](https://medium.com/mozilla-tech/why-rust-is-the-most-loved-language-by-developers-666add782563)
[Rust (programming language) - Wikipedia ](https://en.wikipedia.org/wiki/Rust_(programming_language))
[Self-hosting - Wikipedia ](https://en.wikipedia.org/wiki/Self-hosting)
[The Rust Programming Language ](https://www.rust-lang.org/en-US/)
[Intro to the Rust programming language](https://www.youtube.com/watch?v=agzf6ftEsLU)
[rust learning](https://www.google.co.uk/search?q=rust+learning&sa=X&ved=0ahUKEwjm7vzRt8_bAhVCIcAKHau4B2wQ1QII7AEoBw)



[boringcactus (Melody Horn) ](https://www.boringcactus.com/)
[Monads, Explained Without Bullshit  boringcactus ](https://www.boringcactus.com/2020/07/18/monads-without-the-bullshit.html)
[Windows, Vim, and Python: An Unholy Trinity of Pain  boringcactus ](https://www.boringcactus.com/2018/06/17/windows-vim-python.html)
[Some Thoughts About Work  boringcactus ](https://www.boringcactus.com/2018/07/17/some-thoughts-about-work.html)
[What, Then, Shall We Do?  boringcactus ](https://www.boringcactus.com/2020/07/15/what-then-shall-we-do.html)
[A Survey of Rust GUI Libraries  boringcactus ](https://www.boringcactus.com/2020/08/21/survey-of-rust-gui-libraries.html)
[boringcactus](https://www.youtube.com/channel/UCw0N-UmLylMSnCtHZ7vagBw)
[Projects  boringcactus ](https://www.boringcactus.com/projects/)
[mathphreak/StaffDrops: Plugin In Progress ](https://github.com/mathphreak/StaffDrops)
[mathphreak/AltcoinNetWorth: Tells you the total value of your cryptocurrency holdings. ](https://github.com/mathphreak/AltcoinNetWorth)
[mathphreak/ReliefValve: Prevents dangerous Steam buildups by letting you juggle games between library locations. ](https://github.com/mathphreak/ReliefValve)
[mathphreak ](https://github.com/mathphreak)
[mrandri19/rust-editorconfig: A crate that implements editorconfig ](http://editorconfig.org/). Docs at https://docs.rs/editorconfig/1.0.0/editorconfig/ (https://github.com/mrandri19/rust-editorconfig)
[kinkgarden/kinkgarden: share your kinks with your friends, or the world ](https://github.com/kinkgarden/kinkgarden)
[kink.garden ](https://kink.garden/)
[kink.garden ](https://kink.garden/2a2382b1-781b-41b7-80fe-342c20fc9b96)
[Vore](https://www.google.com/search?q=Vore)
[boringcactus/micro-ci: A tiny CI server ](https://github.com/boringcactus/micro-ci)
[csgo-gsi: rust library for CS:GO's Game State Integration API ](https://sr.ht/~boringcactus/csgo-gsi/)
[gemifedi: a gemini frontend to the fediverse ](https://sr.ht/~boringcactus/gemifedi/)
[Fediverse - Wikipedia ](https://en.wikipedia.org/wiki/Fediverse)
[~boringcactus/crowbar-spec: / - sourcehut git ](https://git.sr.ht/~boringcactus/crowbar-spec/tree)
[~boringcactus/crowbar-spec - sourcehut git ](https://git.sr.ht/~boringcactus/crowbar-spec)
[David Letterman - Borat (Full Interview) HD](https://www.youtube.com/watch?v=Sz3PMfADPdQ)
[areweguiyet](https://www.google.com/search?q=areweguiyet)
[Are we GUI yet? ](https://www.areweguiyet.com/)
[Rust 2021: GUI  Raph Levien’s blog ](https://raphlinus.github.io/rust/druid/2020/09/28/rust-2021.html)
[linebender/druid: A data-first Rust-native UI design toolkit. ](https://github.com/linebender/druid)
[iced-rs/iced: A cross-platform GUI library for Rust, inspired by Elm ](https://github.com/iced-rs/iced)
[Game Engines  Are we game yet? ](https://arewegameyet.rs/ecosystem/engines/)
[rust amethyst vs](https://www.google.com/search?q=rust+amethyst+vs)
[Flocking Boids in Rust: With Piston vs Tetra vs Amethyst vs Bevy](https://www.youtube.com/watch?v=e0n9v565HR4)
[Rust Amethyst vs piston vs bevy](https://www.google.com/search?q=Rust+Amethyst+vs+piston+vs+bevy&gs_lcp=Cgdnd3Mtd2l6EAMyBggAEBYQHjoHCAAQRxCwAzoECCMQJzoFCAAQgARKBAhBGABKBAhGGABQtBZYv1NgvFZoAXACeACAAfoEiAGNEJIBCzAuMy4wLjEuMC4ymAEAoAEByAEIwAEB&sclient=gws-wiz)
[mdbook](https://www.google.com/search?q=mdbook)
[Should I use Amethyst or Piston? : rust ](https://www.reddit.com/r/rust/comments/cudjlt/should_i_use_amethyst_or_piston/)
[Wiki - AGuideToRustGameFrameworks2019 ](https://wiki.alopex.li/AGuideToRustGameFrameworks2019#conclusions)
[Ludum Dare](https://www.google.com/search?q=Ludum+Dare)
[~icefox/RustGameFrameworkGuide browse - sourcehut hg ](https://hg.sr.ht/~icefox/RustGameFrameworkGuide/browse/default/astro_ggez/)
[Wiki - TheStateOfGGEZ2018 ](https://wiki.alopex.li/TheStateOfGGEZ2018#email-email-checkin-my-email)
[quicksilver::tutorials - Rust ](https://docs.rs/quicksilver/0.3.12/quicksilver/tutorials/index.html)
[sourcehut - the hacker's forge ](https://sourcehut.org/)
[ldjam.com  Ludum Dare game jam ](https://ldjam.com/)
[Ggez vs Bevy vs Piston vs Amethyst : rust_gamedev ](https://www.reddit.com/r/rust_gamedev/comments/nwoh2k/ggez_vs_bevy_vs_piston_vs_amethyst/)
[Flocking Boids in Rust: With Piston vs Tetra vs Amethyst vs Bevy : rust ](https://www.reddit.com/r/rust/comments/mn337c/flocking_boids_in_rust_with_piston_vs_tetra_vs/)
[Flocking Boids](https://www.google.com/search?q=Flocking+Boids)
[flock-rs/flock.rs at master · JohnPeel/flock-rs ](https://github.com/JohnPeel/flock-rs/blob/master/src/plugins/bidimensional/flock.rs#L112-L164)
[Learning to Fly: Let's simulate evolution in Rust! (pt 1) ](https://pwy.io/en/posts/learning-to-fly-pt1/)
[bevy/ecs_guide.rs at main · bevyengine/bevy ](https://github.com/bevyengine/bevy/blob/main/examples/ecs/ecs_guide.rs)
[How I Made a 2D Shooting Gallery Game using Rust and Bevy in 5 hours](https://www.youtube.com/watch?v=T1ZT0EkzvgI)
[Amethyst 3D Tutorial - Amethyst 3D Tutorial ](https://crsaracco.github.io/amethyst-3d-tutorial/)
[Introduction - Amethyst Documentation ](https://book.amethyst.rs/book/stable/)
[Cubefield - Play Online Free! ](https://www.cubefield.org.uk/)
[Cubefield: New world record - High score 1.766.210](https://www.youtube.com/watch?v=s80HExhyIng)
[Race The Sun - 2,522,754 Run](https://www.youtube.com/watch?v=eDLfSg3YwVQ)

