


[Bevy](/todos/2021/211214-rust-learn-temp.md)! book: [Bevy - Introduction ](https://bevyengine.org/learn/book/introduction/)
[Bevy - A data-driven game engine built in Rust ](https://bevyengine.org/)
[Introduction - Roguelike Tutorial - In Rust ](https://bfnightly.bracketproductions.com/rustbook/)


```
2022-01-16 23:48:11 kvogel-surface-ubuntu:~/projects/general/dev/games/day4night/bevy ±(master) ✗ 
❯ git clone https://github.com/bevyengine/bevy
Cloning into 'bevy'...
❯ cd bevy 
❯ git checkout latest
Branch 'latest' set up to track remote branch 'latest' from 'origin'.
Switched to a new branch 'latest'
```

```
2022-01-17 06:24:37 kvogel-surface-ubuntu:~/projects/general/dev/games/day4night/bevy/bevy ±(latest) 
❯ cargo run --example breakout
   Compiling mint v0.5.8
...
  --- stderr
  thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "`\"pkg-config\" \"--libs\" \"--cflags\" \"libudev\"` did not exit successfully: exit status: 1\nerror: could not find system library 'libudev' required by the 'libudev-sys' crate\n\n--- stderr\nPackage libudev was not found in the pkg-config search path.\nPerhaps you should add the directory containing `libudev.pc'\nto the PKG_CONFIG_PATH environment variable\nNo package 'libudev' found\n"', /home/kvogel/.cargo/registry/src/github.com-1ecc6299db9ec823/libudev-sys-0.1.4/build.rs:38:41
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: build failed
```
>Libudev Rust Bindings The libudev-sys crate provides declarations and linkage for the libudev C library. Following the *-sys package conventions, the libudev-sys crate does not define higher-level abstractions over the native libudev library functions. Dependencies In order to use the libudev-sys crate, you must have a Linux system with the libudev library installed where it can be found by pkg-config. To install libudev on Debian-based Linux distributions, execute the following command: `sudo apt-get install libudev-dev`. libudev is a Linux-specific package. It is not available for Windows, OSX, or other operating systems.
```
2022-01-17 06:24:57 kvogel-surface-ubuntu:~/projects/general/dev/games/day4night/bevy/bevy ±(latest) 
❯ sudo apt install libudev
...
❯ cargo run --example breakout
```
breakout!


Rust Learning Resources
The goal of this book is to learn Bevy, so it won't serve as a full Rust education. If you would like to learn more about the Rust language, check out the following resources:

The Rust Book: the best place to learn Rust from scratch
Rust by Example: learn Rust by working through live coding examples


[Bevy - A data-driven game engine built in Rust ](https://bevyengine.org/)
[bevy - crates.io: Rust Package Registry ](https://crates.io/crates/bevy)
[Bevy - Setup ](https://bevyengine.org/learn/book/getting-started/setup/)
[A bevy is a group of birds!](https://www.google.com/search?q=A+bevy+is+a+group+of+birds!&ie=UTF-8)

[bevy/examples at latest · bevyengine/bevy ](https://github.com/bevyengine/bevy/tree/latest/examples)
[Bevy - Assets ](https://bevyengine.org/assets/#learning)

[jcornaz/heron: An ergonomic physics API for bevy games. ](https://github.com/jcornaz/heron)
[james7132/bevy-steamworks: A Bevy plugin for integrating with the Steamworks SDK ](https://github.com/james7132/bevy-steamworks)
[Steamworks SDK (Steamworks Documentation) ](https://partner.steamgames.com/doc/sdk)
[ramirezmike/not_snake_game: A snake-inspired game made in Rust using the Bevy game engine. ](https://github.com/ramirezmike/not_snake_game)
[Not Snake by Michael Ramirez ](https://ramirezmike2.itch.io/not-snake)
[TheRealTeamFReSh/MurderUserDungeon: A game made for the Rusty Jam https://itch.io/jam/rusty-jam ](https://github.com/TheRealTeamFReSh/MurderUserDungeon)
[bonsairobo/bevy_rider: Classic Line Rider implemented with the Bevy engine and Rapier 2D. ](https://github.com/bonsairobo/bevy_rider)
[rparrett/taipo: Japanese Typing Tower Defense ](https://github.com/rparrett/taipo)
[notation-fun/notation: A modern visual music notation, colorful and dynamic ](https://github.com/notation-fun/notation)
[bonsairobo/bevy_rider: Classic Line Rider implemented with the Bevy engine and Rapier 2D. ](https://github.com/bonsairobo/bevy_rider)
[BottledByte / Per Spatium · GitLab ](https://gitlab.com/BottledByte/per-spatium)
[SuperiorJT/bevy_pong: Simple pong game made in Bevy Engine ](https://github.com/SuperiorJT/bevy_pong)
[WhoisDavid/bevy-nbody: Experiments with Bevy and n-body problems ](https://github.com/WhoisDavid/bevy-nbody)
[Ephemeris - Wikipedia ](https://en.wikipedia.org/wiki/Ephemeris)
[insrcd/labyrinth-game ](https://github.com/insrcd/labyrinth-game)
[Carter Anderson Bevy,](https://www.google.com/search?q=Carter+Anderson++Bevy%2C&gs_lcp=Cgdnd3Mtd2l6EAM6CggjEK4CELADECdKBAhBGAFKBAhGGABQlTpYlTpguz9oAnAAeACAAXOIAXOSAQMwLjGYAQCgAQHIAQHAAQE&sclient=gws-wiz)
[Carter Anderson (@cart_cart) / Twitter ](https://twitter.com/cart_cart?lang=en)
[cart](https://www.youtube.com/cartdev)

[Carter Anderson on Twitter: "Compare their current track record, structure, and values to Twitter / Facebook / Instagram / WhatsApp / TikTok and absolute dumpster fire that is crypto (as it stands today) and I think the choice is clear." / Twitter ](https://twitter.com/cart_cart/status/1460476676098326541)
[cart](https://www.youtube.com/cartdev)
[cart (Carter Anderson) ](https://github.com/cart)
[Carter Anderson  LinkedIn ](https://www.linkedin.com/in/carter-anderson-4b272866/)
[Read Rust – Posts  Facebook ](https://www.facebook.com/775938652614667/posts/scaling-bevy-by-carter-anderson/1406739779534548/)
[About Me - Hadria Beth Portfolio ](https://hadriabeth.com/about-me/)
[Today's Comics Online  Read Comic Strips at GoComics ](https://www.gocomics.com/)
[cryscan/summer-jam: Pong but has gravity. ](https://github.com/cryscan/summer-jam)
[Bobox214/Kataster: A single screen space shooter developed in Rust with Bevy and Heron ](https://github.com/Bobox214/Kataster)
[bevy/examples at latest · bevyengine/bevy ](https://github.com/bevyengine/bevy/tree/latest/examples#ecs-entity-component-system)
[lmao ](https://caballerocoll.com/blog/bevy-rhythm-game/)
[Creating a Snake Clone in Rust, with Bevy :: A blog — Just a blog ](https://mbuffett.com/posts/bevy-snake-tutorial/)
[Introduction - Unofficial Bevy Cheat Book ](https://bevy-cheatbook.github.io/)
[Gravity and Physics in the Bevy Game Engine // Spencer B. ](https://sburris.xyz/posts/bevy-gravity/)


[WhoisDavid/bevy-nbody: Experiments with Bevy and n-body problems ](https://github.com/WhoisDavid/bevy-nbody)
whoa - planets n stuff

[What are the Rust types denoted with a single apostrophe?](https://stackoverflow.com/questions/22048673/what-are-the-rust-types-denoted-with-a-single-apostrophe)
[Bevy Engine – Addressing the elephant in the room - Community - Amethyst Forums ](https://community.amethyst.rs/t/bevy-engine-addressing-the-elephant-in-the-room/1645)


ECS
[Getting started with the Megacity demo - Unity at GDC 2019](https://www.youtube.com/watch?v=UPnLa0LMbHQ)
pathing? gamedev
dogfooding - production-led development
Unity DOTS, Burst Compiler
position, rotation, scale
Subscenes

ECS
Space has things
inheritance - a class of thing can only have one parent class
ECS
base thing
add components to entity - composition
monobehaviour Unity - like classes?
overhead of access and storage

data-driven
Entity is tagged with unique Hash ID
strictly for data, no functionality
functionality housed in Systems
Systems iterate through any enitty with the required component
e.g. 
Component A
Vector3 position
Quaternion direction
float speed

functionality abstracted into e.g.:
System for anything that can move
System for anything that can be seen
System for anything that can take damage

associative array
 quaternion?
like flyweight pattern
vr - efficiency of ecs will be huge boon to the medium

Nordeus



[Bevy - A data-driven game engine built in Rust ](http://127.0.0.1:1111/)
['Bevy Book' - HTTrack Website Copier ](http://kvogel-surface-ubuntu:8081/server/refresh.html)
[tatarchuk destiny renderer](https://www.google.com/search?q=tatarchuk+destiny+renderer&gs_lcp=Cgdnd3Mtd2l6EAM6BwgAEEcQsAM6BAgAEB46BggAEAUQHjoGCAAQFhAeOgUIIRCgAUoECEEYAEoECEYYAFDpA1ipIWChJWgBcAJ4AIAB0wKIAcEWkgEIMC4xNS4wLjKYAQCgAQHIAQjAAQE&sclient=gws-wiz)
[Slide 1 ](https://advances.realtimerendering.com/destiny/gdc_2015/Tatarchuk_GDC_2015__Destiny_Renderer_web.pdf)
[Destiny’s Multi-threaded Renderer Architecture, GDC 2015, Natalya Tatarchuk ](http://advances.realtimerendering.com/destiny/gdc_2015/)
[Slide 1 ](https://i3dsymposium.org/2015/I3D_Tatarchuk_keynote_2015_for_web.pdf)
