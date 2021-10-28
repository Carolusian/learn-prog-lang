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
- `&` is normally used before type or lifetime type during declaration

### Ch5: Struct

- object's data attributes
- field init shorthand
- struct update syntax: `..another_stuct`
- tuple structs
- structs storing reference data need to use `lifetime`
- `{:?}`, `{:#?}`: calls the `std::fmt::Debug`: need `#[derive(Debug)]`
- methods normally don't take ownership with `self`, mostly using `&self` and `&mut self`
- automatic `referencing` and `dereferencing`
- `associated functions`: still functions, not method (like static method in OOP), invoked using `::`

### Ch6: Enums and Pattern Matching

- for enumerating possible values, similar to algebraic data types
- variants and values: data values can be put directly in variants
- we can define methods for enums
- null references: the billion dollar mistake
- option
- `match` and `if let .. else ..`

### Ch7: Modules

TODO

### Ch9: Error Handling

- `Ok` and `Err` are just enum variants
- unrecoverable: `panic!` macro
- backtrace: `RUST_BACKTRACE=1 cargo run`
- recoverable: `Result` enum with `Ok` and `Err` variants
- shorthands for panic!: `unwrap` and `expect`
- propagating: return `Result<T, E>`
- shorthand for propagating: `?` operator, can chain method calls, and returns Result.
- Dynamic Err dispatch: Result((), Box<dyn Error>)

### Ch10: Generic, Traits, and Lifetimes

- `imp<T> Point<T>` vs `imp Point<f32>` (implement method only on `f32` type)
- `monomorphization`: turning generic code into specific code, by filling in the concrete types during compile
- trait restriction: can only implement a trait on a type only if the trait or the type is local to our crate
- trait bounds
- `blanket implementation`

### Ch17: OOP Features in Rust

- `dyn`: dynamic object with common behavior: `pub components: Vec<Box<dyn Draw>>`

## Reference:

- [A half-hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)
- [Idiomatic Rust](https://github.com/mre/idiomatic-rust)
- [Reading Rust Function Signatures](https://github.com/brson/rust-anthology/blob/master/src/reading-rust-function-signatures.md)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/intro.html)
- [Async Programming in Rust](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)
- [Scala Developer Journey into Rust](http://blog.madhukaraphatak.com/rust-scala-part-1/)
- [Rust VIM + REPL](https://startupsventurecapital.com/rust-ide-repl-in-vim-11daa921a2c4)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Extending Python with Rust](https://developers.redhat.com/blog/2017/11/16/speed-python-using-rust#what_is_rust_)
  - See also PyO3: https://github.com/PyO3/pyo3
- Useful macros:
  - println!, format!, write!, assert!
