
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

[macos - Environment variables in Mac OS X](https://stackoverflow.com/questions/603785/environment-variables-in-mac-os-x/3756674#3756674)
add
```bash
launchctl setenv PATH $PATH
```
to `~/.zshrc`?

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

[Extension doesn't work with zsh / VSCode Mac OSX Catalina · Issue #850 · rust-lang/vscode-rust ](https://github.com/rust-lang/vscode-rust/issues/850)
>This happens because your shell's PATH doesn't apply to Code if you don't start it from a shell (e.g. terminal).
>the Code terminal spawns a shell, which reads ~/.profile or whatever and gets the correct PATH

## done

The "rustsym" command is not available. Make sure it is installed.
Source: Rusty Code (extension) (???) disabled
