# Rust Notes

## Key concepts

### Ownership

### Option & unwrap

- learn by comparing with Scala & F#:
  - [Module std::option](https://doc.rust-lang.org/std/option/): docs is good
  - [Scala Option](https://www.scala-lang.org/api/current/scala/Option.html): used as a collection or monad
  - [F# Option](https://docs.microsoft.com/en-us/dotnet/fsharp/language-reference/options): used as a collection
  - `unwrap_or` is like `getOrElse` in scala
  - `unwrap` is like `unwrap`, but can panic, so it is not encouraged

## Book: The Rust Programming Language

NOTE: can learn just chapter 1-11 for the basics, chapter 4 & 10 is new concepts about ownership, borrowing and lifetime

### Ch1: Installation

- `rustup` and hello world
- `cargo`: build, check, run

### Ch3: Common programming concepts

- Mutable vs shadowing: shadowing allows a different type, spare us from thinking of different names
- Data types: scalar vs compound
- Data types: Integer, float, char, tup, array
- Macro: `!`, e.g. `println!`
- Statement vs expression: expression doesn't end with semicolon

### Ch4: Ownership

- stack vs heap

### Ch5: Struct

- object's data attributes
- field init shorthand
- struct update syntax: `..another_stuct`
- tuple structs
- structs storing reference data need to use `lifetime`
- `{:?}`, `{:#?}`: calls the `std::fmt::Debug`: need `#[derive(Debug)]`
- method normal don't take ownership with `self`, mostly using `&self` and `&mut self`
- automatic `referencing` and `dereferencing`
- `associated functions`: still functions, not method (like static method in OOP), invoked using `::`

## Reference:

- [A half-hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)
- [Idiomatic Rust](https://github.com/mre/idiomatic-rust)
- [Reading Rust Function Signatures](https://github.com/brson/rust-anthology/blob/master/src/reading-rust-function-signatures.md)
- [Scala Developer Journey into Rust](http://blog.madhukaraphatak.com/rust-scala-part-1/)
- [Rust VIM + REPL](https://startupsventurecapital.com/rust-ide-repl-in-vim-11daa921a2c4)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Extending Python with Rust](https://developers.redhat.com/blog/2017/11/16/speed-python-using-rust#what_is_rust_)
  - See also PyO3: https://github.com/PyO3/pyo3
