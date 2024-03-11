# Rlox

From Crafting Interpreters

An implementation of clox from [Crafting Interpreters](https://craftinginterpreters.com/) as a visual studio code extension implemented in rust (wasm).

This is the second part of the book, `A Bytecode Virtual Machine`. For an implementation of the interpreter see [lox](https://github.com/madian44/lox)

## Pre-requisites

- rust toolchain (https://rustwasm.github.io/docs/book/game-of-life/setup.html) (rust and wasm-pack at least)
- node (https://github.com/nvm-sh/nvm)
- vcse (https://code.visualstudio.com/api/working-with-extensions/publishing-extension#vsce)
- make

## Building

Pretty basic at the moment:

    $ cd rlox
    $ make build

For coverage `grcov` and `llvm-tools-preview` are required:

    $ cargo install grcov
    $ rustup component add llvm-tools-preview 

## Projects

### rlox

`rust` implementation of clox.

### rlox-wasm

`wasm` build of `rlox`.

## Progress through the book...

### Implemented

 * Chapter 14 Chunks of Bytecode
 * Chapter 15 A Virtual Machine

