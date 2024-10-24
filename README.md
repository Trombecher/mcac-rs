# Minecraft Anvil Combinator (Rust)

This is a library that provides the functionality to calculate the optimal way to combine items on an anvil.

There exists an [original implementation in TypeScript](https://github.com/Trombecher/mcac). In hopes that one day I'll be able to target WASM, I rewrote it in Rust, then forgot about it for a year and then rewrote it again in Rust, making it much faster. You can check out the initial commit if you want to look at the initial implementation.

## How To Build This Project

First of all you need Rustup (or a valid Rust installation). Then you'll need a JavaScript runtime. Why? Because I say so.

To compile, you need some pre-generated Rust code. To generate that code, pipe the stdout of the invocation of [js/gen.js](https://github.com/Trombecher/mcac-rs/blob/main/js/gen.js) to src/dist.rs, just like this:

```shell
bun run js/gen.js > src/dist.rs
```

Then you can build the project using:

```shell
cargo build
```

or in release mode:

```shell
cargo build --release
```