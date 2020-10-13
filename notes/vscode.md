

[Setting up a Rust Development Environment ](https://hoverbear.org/blog/setting-up-a-rust-devenv/)
[macos - Environment variables in Mac OS X](https://stackoverflow.com/questions/603785/environment-variables-in-mac-os-x/3756674#3756674)

### (official?) Rust extension

[Rust - Visual Studio Marketplace ](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)

seems slow to pick up changes - one save behind? i.e. when you fix something, the error/warning remains, until you save again - even just by adding and deleting a random character

## done

### Rust (official) extn problems

sorted somehow - by uninstalling it and 2 other Rust extensions, and re-installing?

vscode rls extn
>Rust for Visual Studio Code (powered by Rust Language Server/Rust Analyzer). Provides lints, code completion and navigation, formatting and more.
>This extension is enabled globally.

>Couldn't start client Rust Language Server

>Could not install component: `rust-analysis` Source: Rust (extension)
click notification to install:
```
> Executing task: rustup component add rust-analysis --toolchain stable-x86_64-apple-darwin <
zsh:1: command not found: rustup
The terminal process "/bin/zsh '-c', 'rustup component add rust-analysis --toolchain stable-x86_64-apple-darwin'" failed to launch (exit code: 127).
Terminal will be reused by tasks, press any key to close it.
```
run from integrated terminal:
```
20/10/12 21:09:12 kvogel-macbook-2018:~/pl/learn-rust ±(master) ✗ 
❯ rustup component add rust-analysis --toolchain stable-x86_64-apple-darwin
info: downloading component 'rust-analysis'
info: installing component 'rust-analysis'
```

but again (opening `.rs` file in vscode):
```
> Executing task: rustup component add rust-analysis --toolchain stable-x86_64-apple-darwin <
zsh:1: command not found: rustup
The terminal process "/bin/zsh '-c', 'rustup component add rust-analysis --toolchain stable-x86_64-apple-darwin'" failed to launch (exit code: 127).
Terminal will be reused by tasks, press any key to close it.

> Executing task: rustup component add rust-src --toolchain stable-x86_64-apple-darwin <
zsh:1: command not found: rustup
The terminal process "/bin/zsh '-c', 'rustup component add rust-src --toolchain stable-x86_64-apple-darwin'" failed to launch (exit code: 127).
Terminal will be reused by tasks, press any key to close it.
```
```
20/10/12 21:13:13 kvogel-macbook-2018:~/pl/learn-rust/crash-course/hello2 ±(master) ✗ 
❯ rustup component add rust-src --toolchain stable-x86_64-apple-darwin
info: downloading component 'rust-src'
info: installing component 'rust-src'
```
```
20/10/12 21:20:56 kvogel-macbook-2018:~/pl/learn-rust/crash-course/hello2 ±(master) ✗ 
❯ rustc --print=sysroot
/Users/kvogel/.rustup/toolchains/stable-x86_64-apple-darwin
```

[zsh:1: command not found: rustup](https://www.google.com/search?q=zsh%3A1%3A+command+not+found%3A+rustup&ie=UTF-8)
[Rustup command not found error in terminal - help - The Rust Programming Language Forum ](https://users.rust-lang.org/t/rustup-command-not-found-error-in-terminal/30918/9)
[Working with MacOS Catalina zsh - help - The Rust Programming Language Forum ](https://users.rust-lang.org/t/working-with-macos-catalina-zsh/33592)
>`rustup` by default adds `~/.cargo/bin` to your `PATH` in `.profile`. It sounds like zsh is not evaluating your `.profile`. (Terminal windows on Mac count as login shells, iirc, and so would normally evaluate `.profile`.)
>Thanks a lot, got fixed with me by doing: `ln -s .profile .zprofile`

2020-10-13 16:25:48
(uninstalled spacevim)
```
20/10/13 16:16:47 kvogel-macbook-2018:~/pl/learn-rust/crash-course/hello2 ±(master) ✗ 
❯ view -l  ~/.zprofile
```
```bash
# Setting PATH for Python 2.7
# The original version is saved in .zprofile.pysave
PATH="/Library/Frameworks/Python.framework/Versions/2.7/bin:${PATH}"
export PATH

export PATH="$HOME/.cargo/bin:$PATH"
```

[zsh - /home/user/.zshenv:5: command not found: rustc - Unix & Linux Stack Exchange ](https://unix.stackexchange.com/questions/466685/home-user-zshenv5-command-not-found-rustc)
[bash - Make rustc, cargo, rustup, rustdoc commands work without sudo inside the Windows Subsystem for Linux](https://stackoverflow.com/questions/50586535/make-rustc-cargo-rustup-rustdoc-commands-work-without-sudo-inside-the-windows)
[rustup not available vscode](https://www.google.com/search?q=rustup+not+available+vscode&ved=2ahUKEwi1sN7r7LHsAhWIecAKHcH8DhcQ1QIoAHoECAsQAQ)
[Error start RLS in version 0.6.1 with message "Rustup not available" · Issue #577 · rust-lang/vscode-rust ](https://github.com/rust-lang/vscode-rust/issues/577)
[visual studio code - Couldn't start client Rust Language Server](https://stackoverflow.com/questions/58312319/couldnt-start-client-rust-language-server)
[Setting up Rust development environment using VSCode on a Mac - Adventurous Computing ](https://blog.cyplo.net/posts/2017/11/rust-vscode-mac/)
[Updating rustup / installing rls directly broke the vscode rust extension : rust ](https://www.reddit.com/r/rust/comments/7la5oi/updating_rustup_installing_rls_directly_broke_the/)
[RLS and Rustup errors on Code-Server · Issue #814 · rust-lang/vscode-rust ](https://github.com/rust-lang/vscode-rust/issues/814)
[Complete Rust plugin install for VS Code via command line](https://stackoverflow.com/questions/59135678/complete-rust-plugin-install-for-vs-code-via-command-line)
[How do I solve "Couldn't start client Rust Language Server" with the Rust VS Code extension?](https://stackoverflow.com/questions/60816886/how-do-i-solve-couldnt-start-client-rust-language-server-with-the-rust-vs-cod)

[macos - Environment variables in Mac OS X](https://stackoverflow.com/questions/603785/environment-variables-in-mac-os-x/3756674#3756674)
add
```bash
launchctl setenv PATH $PATH
```
to `~/.zshrc`? no dice

is `$HOME/.cargo/bin` in PATH? Yes:
```
20/10/12 21:29:54 kvogel-macbook-2018:~/pl/learn-rust/crash-course/hello2 ±(master) ✗ 
❯ grep cargo ~/.* 2>/dev/null
/Users/kvogel/.bash_profile:export PATH="$HOME/.cargo/bin:$PATH"
/Users/kvogel/.gitignore:# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
/Users/kvogel/.profile:export PATH="$HOME/.cargo/bin:$PATH"
/Users/kvogel/.viminfo:?/cargo
/Users/kvogel/.viminfo:|2,1,1602534505,47,"cargo"
/Users/kvogel/.zprofile:export PATH="$HOME/.cargo/bin:$PATH"
...
```
`rustup`, `rustc`, `cargo` all work from the terminal, in VSCode or not.

[Extension doesn't work with zsh / VSCode Mac OSX Catalina · Issue #850 · rust-lang/vscode-rust ](https://github.com/rust-lang/vscode-rust/issues/850)
>This happens because your shell's PATH doesn't apply to Code if you don't start it from a shell (e.g. terminal).
>the Code terminal spawns a shell, which reads ~/.profile or whatever and gets the correct PATH

```
20/10/12 23:21:42 kvogel-macbook-2018:~/p/general/dev ±(master) ✗ 
❯ which -a rustup
/Users/kvogel/.cargo/bin/rustup
❯ rustup --version
rustup 1.21.1 (7832b2ebe 2019-12-20)
❯ rustup self update
info: checking for self-updates
info: downloading self-update
info: rustup updated successfully to 1.22.1
20/10/12 23:38:00 kvogel-macbook-2018:~/pl/learn-rust ±(master) ✗ 
❯ rustup update
info: syncing channel updates for 'stable-x86_64-apple-darwin'
info: latest update on 2020-10-08, rust version 1.47.0 (18bf6b4f0 2020-10-07)
...
info: installing component 'rust-src'
info: Defaulting to 500.0 MiB unpack ram
info: installing component 'rust-analysis'
info: installing component 'rustc'
info: installing component 'rust-std'
info: installing component 'cargo'
info: installing component 'rust-docs'
info: checking for self-updates
  stable-x86_64-apple-darwin updated - rustc 1.47.0 (18bf6b4f0 2020-10-07) (from rustc 1.44.0 (49cae5576 2020-06-01))
info: cleaning up downloads & tmp directories
```

VSCode prefs (`cmd+,`)
>Rust-client: Disable Rustup. Disable usage of rustup and use rustc/rls from PATH.

```
20/10/12 23:38:56 kvogel-macbook-2018:~/pl/learn-rust ±(master) ✗ 
❯ which rls
/Users/kvogel/.cargo/bin/rls
20/10/12 23:51:43 kvogel-macbook-2018:~/pl/learn-rust ±(master) ✗ 
❯ rls
error: 'rls' is not installed for the toolchain 'stable-x86_64-apple-darwin'
To install, run `rustup component add rls --toolchain stable-x86_64-apple-darwin`
20/10/12 23:51:45 kvogel-macbook-2018:~/pl/learn-rust ±(master) ✗ 
❯ rustup component add rls --toolchain stable-x86_64-apple-darwin
info: downloading component 'rls'
info: installing component 'rls'
info: Defaulting to 500.0 MiB unpack ram
```

### rust-analyzer

uninstalled official Rust extension, installed [rust-analyzer/rust-analyzer: An experimental Rust compiler front-end for IDEs ](https://github.com/rust-analyzer/rust-analyzer)
  lints but doesn't autocomplete...
>rust-analyzer failed to discover workspace

[Rust Analyzer won't work! - Editors and IDEs - The Rust Programming Language Forum ](https://users.rust-lang.org/t/rust-analyzer-wont-work/43712)
[rust-analyzer/rust-analyzer: An experimental Rust compiler front-end for IDEs ](https://github.com/rust-analyzer/rust-analyzer)

### How many Rust extensions??

3:
```
20/10/13 15:58:11 kvogel-macbook-2018:~/.local
❯ ls ~/.vscode/extensions/*rust*
/Users/kvogel/.vscode/extensions/matklad.rust-analyzer-0.2.344:
README.md                      package-lock.json              rust.tmGrammar.json
icon.png                       package.json
out/                           ra_syntax_tree.tmGrammar.json

/Users/kvogel/.vscode/extensions/rust-lang.rust-0.7.8:
CHANGELOG.md                 cmd/                         package.json
COPYRIGHT                    contributing.md              rust-icon.png
LICENSE-APACHE               language-configuration.json  snippets/
LICENSE-MIT                  node_modules/
README.md                    out/

/Users/kvogel/.vscode/extensions/saviorisdead.rustycode-0.19.1:
CHANGELOG.md  README.md     gulpfile.js   node_modules/ package.json  tslint.json
LICENSE       ROADMAP.md    images/       out/          snippets/
```
wtf is `rustycode`?
```
20/10/13 15:58:23 kvogel-macbook-2018:~/.local
❯ view ~/.vscode/extensions/saviorisdead.rustycode-0.19.1/README.md

# Rust for Visual Studio Code (Latest: 0.19.1)
https://github.com/saviorisdead/RustyCode/
```
2020-10-13 18:47:41
uninstalled "Rusty Code"? syntax hightlighter, then rust-analyzer, reload, installed official Rust extn
seemed to work, briefly...
now kind of works??

[Can't find rustup · Issue #231 · editor-rs/vscode-rust ](https://github.com/editor-rs/vscode-rust/issues/231)
[Rustup no longer detected · Issue #242 · editor-rs/vscode-rust ](https://github.com/editor-rs/vscode-rust/issues/242)
[RLS now available via rustup : rust ](https://www.reddit.com/r/rust/comments/660jii/rls_now_available_via_rustup/)
[Can't find rustup · Issue #231 · editor-rs/vscode-rust ](https://github.com/editor-rs/vscode-rust/issues/231)
[How did you set up your vscode? : rust ](https://www.reddit.com/r/rust/comments/7jslij/how_did_you_set_up_your_vscode/)

[rust-lang/vscode-rust: Rust extension for Visual Studio Code ](https://github.com/rust-lang/vscode-rust)
[Issues · rust-lang/vscode-rust ](https://github.com/rust-lang/vscode-rust/issues)
[Extension doesn't work with zsh / VSCode Mac OSX Catalina · Issue #850 · rust-lang/vscode-rust ](https://github.com/rust-lang/vscode-rust/issues/850)

[Managing Extensions in Visual Studio Code ](https://code.visualstudio.com/docs/editor/extension-gallery)
[rls/debugging.md at master · rust-lang/rls ](https://github.com/rust-lang/rls/blob/master/debugging.md)
[rustup not available (linux + zsh) · Issue #675 · rust-lang/vscode-rust ](https://github.com/rust-lang/vscode-rust/issues/675)
[Rustup no longer detected · Issue #242 · editor-rs/vscode-rust ](https://github.com/editor-rs/vscode-rust/issues/242)

[RLS Cannot be started. · Issue #237 · rust-lang/vscode-rust ](https://github.com/rust-lang/vscode-rust/issues/237)

[vs code rust extension not working on mac](https://www.google.com/search?q=vs+code+rust+extension+not+working+on+mac&ie=UTF-8)



The "rustsym" command is not available. Make sure it is installed. Source: Rusty Code (extension) (???)
  Rusty Code extension disabled
