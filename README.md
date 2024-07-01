# WASI Rust C++ Linking

This code present a basic way of building a c++ library and bind it to rust in order to compile to WASI.

## Description

This is composed of 2 crates:
- [cool-sys](./cool-sys/): This is the crate for C++ bindings following [conventions](https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key).
- [cool](./cool/): This is the crate of the main executable that is depending on cool-sys.

## Build

In order to build this for WASI, we can simply use [cc-rs](https://docs.rs/cc/latest/cc/), which should support WASI out of the box.
