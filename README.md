 Probly not helpful to anyone, but wanted to record my learning.

## methodology

I went through every page of [the book](https://doc.rust-lang.org/book/) and wrote any code examples they had, and took notes. I primarily use js & python, and have recent practical experiencce with java, C++, and a few others, so my notes are sort of from that perspective. I prefixed them all with letters (`aa`, `ab`, etc) to keep them in order, as you can't start a crate with a digit.

After that, I made [a rust module](https://dev.to/ghost/rust-project-structure-example-step-by-step-3ee) that can start/stop a socket thread. I structured everything using guidance from [the cargo book](https://doc.rust-lang.org/cargo/guide/). I included unit-tests, Gitlab CI to build for various platforms, and docs to describe the API.

Then I exposed that lib to C (for FFI in lua) including generating a `.h`file and a test lua program that uses the bindings.

## usage

Start a new project like this:

```
cargo new NAME
```

Inside the dir, run it with `cargo run` or build release version with `cargo build --release`

`Cargo.toml` tracks dependencies, which you have to manually add (not like `npm i`)

YOu can clean up code with `cargo fix`
