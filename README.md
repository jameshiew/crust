# crust

Writing a basic C compiler in Rust. Tests are in a git submodule.

I was following along with [Nora Sandler's "Writing A C Compiler"](https://norasandler.com/2017/11/29/Write-a-Compiler.html) series. Currently aiming to pass Week 1 tests, but currently does not work perfectly and needs to be reconfigured to emit 64-bit rather than 32-bit assembly.

## Prerequisites
* Rust
* gcc

## Quickstart
```bash
cargo build --release
target/release/crust tests/stage_1/valid/return_0.c
```
