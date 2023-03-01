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

### Tips

- Quick handson with less Error handling, e.g. panic, and unwrap, then can refactor
- Don't over design, it make the handson coding slow and different, because you cater too many things

### Cross Compiling

MacOS cross compile to Windows for example

- `brew install mingw-w64`
- `rustup target add x86_64-pc-windows-gnu`
- `cargo build --release --target x86_64-pc-windows-gnu`

## Book: The Rust Programming Language

NOTE: can learn just chapter 1-11 for the basics, chapter 4 & 10 is new concepts about ownership, borrowing and lifetime

### Ch1: Installation

- `rustup` and hello world
- `cargo`: build, check, run
- `cargo install cargo-edit`, then `cargo add {package_name}`

### Ch3: Common programming concepts

- Mutable vs shadowing: shadowing allows a different type, spare us from thinking of different names
- Data types: scalar vs compound
- Data types: Integer, float, char, tup, array
- Macro: `!`, e.g. `println!`
- Statement vs expression: expression doesn't end with semicolon

### Ch4: Ownership

- stack vs heap
- `*` is the dereferencing operator
- `&` create a reference which does not own the borrowed value
- `&` is normally used before type or lifetime type during declaration
- `&mut var`: exclusive borrowing, only one mutable reference is allowed at the same time
- `&var`: shared borrowing, multiple borrowers are allowed
- `Slices` slices let you reference a contiguous sequence of elements in a collection

### Ch5: Struct

- object's data attributes
- field init shorthand
- struct update syntax: `..another_stuct`
- **tuple structs**
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

- a `package` is a bundle of one or more `crates`, defined in `Cargo.toml`
- `modules` cheatsheet:
  - `crate root` > declaring modules (module.rs or module/mod.rs) > declaring submodules
  - path of modules: `crate::modulde::submodule::Struct_or_fn`
  - private vs public
  - `use`: use module code in shortcut, can be combined with curly braces
  - `pub use`: import and re-export

### Ch8: Common Collections

- `vector`, `String`, `hash map`
- `vector`:
  - `vec!` macro is syntax sugar to create new vector with initial values
  - `[] vs get`: for reading element
  - `for i in &mut v`: and dereference operator
  - use Enum to store multiple types in vector
- `String`:
  - `String::from("initial contents")`
  - `push_str` and `push`: string slice and character
  - `"3A".chars()` vs `"3A".bytes`
- ## `hash map`:
  - `use std::collections::HashMap`
  - `.copied` to convert `Option<&i32>` to `Option<i32>`
  - `for (key, value) in &scores`

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

### Ch13: Iterators and Closures

- `closures`: able to infer types reliably
- `move`: to force take ownership
- `Iterator` trait: `next` method, return Some(Self::Item) or None
- Type `Item`: associated type to the trait
- consumer methods: consuming adaptors, `sum`, `collect`
- producer methods: iterator adaptors, lazy, have to call consumer methods
- iterator is fast, zero cost abstraction

### Ch14: Cargo

- [profile.dev] vs [profile.release]: different profile settings, compile time affected
- workspace: managing multiple workspace

### Ch15: Smart Pointer

- `Box::new(5)`: store data on head rather than stack without performance overhead
  - unknow size during compilation
  - transfer ownership of large data without copying
  - want to own value of a particular trait rather than type
- `cons list`: lisp expression from Lisp
- `indirection`: storing a pointer instead of a value
- implicit Deref Coercions: `*y` equals to `*(y.deref())`, see `std::ops::Deref` and `DerefMut` trait
- `Drop` trait
- `Rc<T>`: like Box, and enable multiple ownership
- `RefCell<T>`: like Box, enable interior mutability

## Ch16: Fearless Concurrency

- `move`: for the closures in the threads to take ownership, so main thread cannot somehow invalidate it
- message passing: do not communicate by sharing memory, share by communicate
- `std::sync::mpsc`: multiple producer single consumer:
  - `let (tx, rx) = mpsc::channel();`
  - the above defines a transmitter and a receiver
  - combine `loop` with `try_recv` as it does not block
  - `rx.recv()` or treat `rx` as iterator using `for received in rx`
- `Mutex` for mutual exclusion, only one thread to access at any given time
  - `Arc<T>` Atomic Reference Counting
  - Arc is also interior mutable
  - See: how to count from 1 to 10 with threads safely

### Ch17: OOP Features in Rust

- `dyn`: dynamic object with common behavior: `pub components: Vec<Box<dyn Draw>>`

### Ch20: Final Project - Building a Multithreaded Web Server

- compiler driven development: `cargo check`

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
  - println!, format!, write!, assert!, eprintln!, todo!, asm!, global_asm!
  - trace!, debug!, warn!, info! (in log crate)
- [Common traits](https://stevedonovan.github.io/rustifications/2018/09/08/common-rust-traits.html):
  - ToString, fmt::Display, #[derive(Debug)], Default, From/Into, Clone/Copy, FromStr
- [Rust + Electron + FFI](https://titanwolf.org/Network/Articles/Article?AID=166184ad-585a-4d9b-b755-b1160613cb06)
- [Sample project: plagiarism checker](https://github.com/frizensami/plagiarism-basic)
- [Rust: Recommended VSCode Extensions to Setup Your IDE](https://www.becomebetterprogrammer.com/rust-recommended-vscode-extensions/)
  - Rust-analyzer / Better TOML / CodeLLDB / Crates / Error Lens
- [From and Into](https://rustwiki.org/zh-CN/rust-by-example/conversion/from_into.html)
