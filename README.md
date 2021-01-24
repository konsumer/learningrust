 Probly not helpful to anyone, but wanted to record my learning.

I went through every page of [the book](https://doc.rust-lang.org/book/) and wrote any code examples they had, and took notes.

After that, I made a rust lib that can start/stop a socket thread.

Then I exposed that lib to C (for FFI in lua.)

## methodology

I prefixed them all with letters (`aa`, `ab`, etc) to keep them in order, as you can't start a crate with a digit.

I primarily use js & python, and have recent practical experiencce with java, C++, and a few others, so my notes are sort of from that perspective.

Start a new project like this:

```
cargo new NAME
```

Inside the dir, run it with `cargo run` or build release version with `cargo build --release`

`Cargo.toml` tracks dependencies, which you have to manually add (not like `npm i`)
