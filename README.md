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

## Things You Can Change

In js/gen.js there is a variable called `MAX_ITEMS`, which dictates the maximum number of items this project can handle (there are a few bugs, where this limit is not checked). If you change that and regenerate the dist.rs file, then the value changes for all of Rust.

## _Why Are You Using JavaScript To Generate Rust Code?_

First of all, I am generating a tailored version of the power set to work with my algorithm. It is a lookup table that specifies all possibilities to partition a list of items into two parts (disregarding mirror images). To me that is not possible with standard Rust macro_rules!, and would need a proc macro at minimum. But I am too lazy to write a proc macro, so I do it with JavaScript.