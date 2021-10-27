
[Amethyst - The open source, data-driven game engine ](https://amethyst.rs/)
[Amethyst Docs ](https://book.amethyst.rs/stable/)
[Getting Started - Amethyst Documentation ](https://book.amethyst.rs/book/stable/getting-started)

```

2021-06-05 00:26:16 kvogel-surface:~/projects/general/dev/learn/learn-rustlang/amethyst/amethyst-starter-2d ±(master) ✗ 
❯ cargo run

error: failed to run custom build command for `alsa-sys v0.1.2`

Caused by:
  process didn't exit successfully: `/home/kvogel/projects/general/dev/learn/learn-rustlang/amethyst/amethyst-starter-2d/target/debug/build/alsa-sys-1b2746d74a9e79cd/build-script-build` (exit code: 101)
  --- stderr
  thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "`\"pkg-config\" \"--libs\" \"--cflags\" \"alsa\"` did not exit successfully: exit code: 1\n--- stderr\nPackage alsa was not found in the pkg-config search path.\nPerhaps you should add the directory containing `alsa.pc\'\nto the PKG_CONFIG_PATH environment variable\nNo package \'alsa\' found\n"', /home/kvogel/.cargo/registry/src/github.com-1ecc6299db9ec823/alsa-sys-0.1.2/build.rs:4:38
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: build failed

2021-06-05 00:33:50 kvogel-surface:/mnt/c/Users/chris
❯ sudo apt install libasound2-dev
...
❯ cargo run
...
❯ sudo apt install pkg-config libasound2-dev libssl-dev cmake libfreetype6-dev libexpat1-dev libxcb-composite0-dev

    Finished dev [unoptimized + debuginfo] target(s) in 40.44s
     Running `target/debug/amethyst-starter-2d`
[INFO][amethyst::app] Initializing Amethyst...
[INFO][amethyst::app] Version: 0.15.3
[INFO][amethyst::app] Platform: x86_64-unknown-linux-gnu
[INFO][amethyst::app] Amethyst git commit: UNKNOWN
[INFO][amethyst::app] Rustc version: 1.52.1 Stable
[INFO][amethyst::app] Rustc git commit: 9bc8c42bb2f19e745a63f3445f1ac248fb015e53
thread 'main' panicked at 'Failed to initialize any backend! Wayland status: XdgRuntimeDirNotSet X11 status: XOpenDisplayFailed', /home/kvogel/.cargo/registry/src/github.com-1ecc6299db9ec823/winit-0.19.5/src/platform/linux/mod.rs:467:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```
[Download VcXsrv Windows X Server from SourceForge.net ](https://sourceforge.net/projects/vcxsrv/files/vcxsrv/1.20.9.0/vcxsrv-64.1.20.9.0.installer.exe/)
Disable access control!

```
[ERROR][gfx_backend_vulkan] 
GENERAL [Loader Message (0)] : setupLoaderTermPhysDevs:  Failed to detect any valid GPUs in the current config
object info: (type: INSTANCE, hndl: 94730358755184)

[ERROR][gfx_backend_vulkan] 
GENERAL [Loader Message (0)] : setupLoaderTrampPhysDevs:  Failed during dispatch call of 'vkEnumeratePhysicalDevices' to lower layers or loader to get count.
object info: (type: INSTANCE, hndl: 94730358755184)

[ERROR][gfx_backend_vulkan] 
GENERAL [Loader Message (0)] : setupLoaderTermPhysDevs:  Failed to detect any valid GPUs in the current config
object info: (type: INSTANCE, hndl: 94730358755184)

[ERROR][gfx_backend_vulkan] 
GENERAL [Loader Message (0)] : setupLoaderTrampPhysDevs:  Failed during dispatch call of 'vkEnumeratePhysicalDevices' to lower layers or loader to get count.
object info: (type: INSTANCE, hndl: 94730358755184)

[ERROR][gfx_backend_vulkan] Could not enumerate physical devices! Initialization of a object has failed
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ErrorMessage { msg: "No physical devices found" }', /home/kvogel/.cargo/registry/src/github.com-1ecc6299db9ec823/amethyst_rendy-0.15.3/src/system.rs:148:81
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

2021-06-05 01:24:19 kvogel-surface:~/projects/general/dev/learn/learn-rustlang/amethyst/amethyst-starter-2d ±(master) ✗ 
❯ sudo apt update
❯ sudo apt install nvidia-settings vulkan-utils


2021-06-05 01:24:52 kvogel-surface:~/projects/general/dev/learn/learn-rustlang/amethyst/amethyst-starter-2d ±(master) ✗ 
❯ vulkaninfo
error: XDG_RUNTIME_DIR not set in the environment.
/build/vulkan-tools-KEbD_A/vulkan-tools-1.2.131.1+dfsg1/vulkaninfo/vulkaninfo.h:477: failed with ERROR_INITIALIZATION_FAILED
2021-06-05 01:29:09 kvogel-surface:~/projects/general/dev/learn/learn-rustlang/amethyst/amethyst-starter-2d ±(master) ✗ 
❯ VK_LOADER_DEBUG=all vulkaninfo 
DEBUG: ReadDataFilesInSearchPaths: Searching the following paths for manifest files: /etc/xdg/vulkan/implicit_layer.d:/etc/vulkan/implicit_layer.d:/usr/local/share/vulkan/implicit_layer.d:/usr/share/vulkan/implicit_layer.d:/home/kvogel/.local/share/vulkan/implicit_layer.d

INFO: Found manifest file /usr/share/vulkan/implicit_layer.d/VkLayer_MESA_device_select.json, version "1.0.0"
```


Window
    amethyst/amethyst-starter-2d: Seed project for 2D games (https://github.com/amethyst/amethyst-starter-2d)
        Package alsa was not found in the pkg-config search path - Google Search (https://www.google.com/search?q=Package+alsa+was+not+found+in+the+pkg-config+search+path&rlz=1C1ONGR_en-GBGB953GB953&oq=Package+alsa+was+not+found+in+the+pkg-config+search+path&aqs=chrome..69i57.574j0j7&sourceid=chrome&ie=UTF-8)
            Build error: package alsa was not found in the pkg-config search path. · Issue #100 · ggez/ggez (https://github.com/ggez/ggez/issues/100)
            apt - No package 'alsa' found in Ubuntu 18.04 - Ask Ubuntu (https://askubuntu.com/questions/1281211/no-package-alsa-found-in-ubuntu-18-04)
            [SOLVED] No package 'alsa' found on Ubuntu 18.04 (https://www.linuxquestions.org/questions/linux-newbie-8/no-package-%27alsa%27-found-on-ubuntu-18-04-a-4175683339/)
    No package 'freetype2' found - Google Search (https://www.google.com/search?q=No+package+%27freetype2%27+found&rlz=1C1ONGR_en-GBGB953GB953&oq=No+package+%27freetype2%27+found&aqs=chrome..69i57j69i64l2.753j0j7&sourceid=chrome&ie=UTF-8)
        Ubuntu - installation fails with "No package freetype2 found" · Issue #1193 · alacritty/alacritty (https://github.com/alacritty/alacritty/issues/1193)
        [Error] : No package 'freetype2' found · Issue #62 · Aloxaf/silicon (https://github.com/Aloxaf/silicon/issues/62)
            /usr/bin/ld: cannot find xcb-render - Google Search (https://www.google.com/search?q=%2Fusr%2Fbin%2Fld%3A+cannot+find+xcb-render&rlz=1C1ONGR_en-GBGB953GB953&sxsrf=ALeKk015i9H7MWvaa1UFCCCvtd9ArA8JTw%3A1622851167179&ei=X766YL6yCvaZjLsPie6g-AQ&oq=%2Fusr%2Fbin%2Fld%3A+cannot+find+xcb-render&gs_lcp=Cgdnd3Mtd2l6EAM6BwgAEEcQsANQ6UBY7UFggkZoAXACeACAAUqIAa0BkgEBM5gBAKABAaoBB2d3cy13aXrIAQjAAQE&sclient=gws-wiz&ved=0ahUKEwj-3d7al__wAhX2DGMBHQk3CE8Q4dUDCA4&uact=5)
                Missing Libraries on Linux with Rust and Amethyst - Stack Overflow (https://stackoverflow.com/questions/55758892/missing-libraries-on-linux-with-rust-and-amethyst)

