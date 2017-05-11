Introduction to Rust
====================

This repository contains material for Rust workshop which was held in [Progressbar hackerspace](https://progressbar.sk).

Setup
-----

To install Rust run this: `curl https://sh.rustup.rs -sSf | sh`

If you don't want to install anything yet, you can use [Rust playground](https://play.rust-lang.org) for simple experiments.

Rust is not just language, it comes with great tools. Some of them:

* cargo - go-to build system and dependency manager. Can't be recommended enough.
* rustdoc (ideally invoked via `cargo doc`) - documentation generator (something like Doxygen)
* clippy - advanced linter. It can teach you best practices by warning you about common mistakes.
* vim plugin - vim plugin, obviously.
* racer and RLS - enable your IDE to suggest completions etc.

I recommend installing vim plugin (use Vundle!) or appropriate Rust plugin for your IDE. One of the most popular IDEs is Visual Studio **Code**.

Creating project
----------------

To create project go into desired directory and run: `cargo new --bin rust_intro`. This will create a directory `rust_intro` with a project of the same name and configuration for building executables. It will also run `git init` and add `.gitignore` on it's own if it's not in git repository already.

Project structure:

```
rust_intro
 +- src/
 |   +- main.rs
 +- target/ (created during build)
 |   +- ... (lot of stuff, not interesting too much)
 +- Cargo.toml
 +- Cargo.lock (created after build)
 +- .git
 +- .gitignore
```

`src/main.rs` contains actual code. After opening it we see:

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo prepared a "Hello world" template for us! Let's try to run it:

```
rust_intro$ cargo run
   Compiling rust_intro v0.1.0 (file:///home/martin/rust_intro)
    Finished dev [unoptimized + debuginfo] target(s) in 2.1 secs
     Running `target/debug/rust_intro`
Hello, world!
rust_intro$
```

It works!

Note that it is running from `target/debug/`. The debug builds are much slower than release build! To make it faster we need to build it with `--release` flag.

Let's get back to the code:

```rust
fn main() {
```

This is obviously a function - denoted by `fn` keyword. `main` suggests entry point and, indeed, it is. (There are other ways entry point can look like, but that's not in scope of this workshop.)


```rust
    println!("Hello, world!");
```

This looks like function call. It is **not**. The exclamation mark denotes macro invocation. Macros can be surprising, that's why exclamation mark. They actually generate new source code from their arguments using obscure rules. In contrast to C(++) macros, they are hygienic. They wash themselves each day and regularly change clothes. Well, actually, they just try hard to not get dirty by surrounding code, so the only identifiers they accept are those provided via arguments.

```
}
```

Try to guess this one, is it end of `main()` function or it isn't?














































Of course it is.

Now, let's look at something [more interesting](01_variables_example/README.md).
