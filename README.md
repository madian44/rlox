# Rlox

From Crafting Interpreters

An implementation of clox from [Crafting Interpreters](https://craftinginterpreters.com/) as a visual studio code extension implemented in rust (wasm).

This is the second part of the book, `A Bytecode Virtual Machine`. For an implementation of the interpreter see [lox](https://github.com/madian44/lox)

## Pre-requisites

- rust toolchain (https://www.rust-lang.org/tools/install)
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

## Progress through the book...

### Implemented

 * Chapter 14 Chunks of Bytecode

