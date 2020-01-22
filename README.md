

# rust-demo

Hands-on demo of the Rust programming language.


## History of Rust

- Originally designed by Graydon Hoare at Mozilla Research, with contributions from Dave Herman, Brendan Eich, and others.
- Mozilla began sponsoring the project in 2009.
- Announced 2010: http://venge.net/graydon/talks/intro-talk-2.pdf
- Rust 1.0, the first stable release, released in 2015.
- Since 2011 `rustc` is compiled using `rustc`.
- `rustc` uses LLVM as its back end.
- The History of Rust: https://www.youtube.com/watch?v=79PSagCD_AY
- Firefox (components Servo and Quantum) written in Rust.
- Ad block engine in Brave.
- `exa`, `ripgrep`, `bat`, and `fd` written in Rust (you really want these tools!).


## Why Rust

- "A language empowering everyone to build reliable and efficient software." (https://www.rust-lang.org)
- Fast but also safe
- Zero cost abstractions
- Type system
- Compiler catches most errors (if it compiles, it often just works)
- Compiler provides helpful error messages
- Memory safety
- Thread safety
- Private and immutable by default
- Great tooling (testing, documentation, auto-formatter, dependency management, package registry, no makefiles needed)
- Community
- Most-loved programming language in the 2016, 2017, 2018, and 2019 Stack Overflow annual surveys


## Great resources

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
- [Why try Rust for scientific computing?](https://erambler.co.uk/blog/why-give-rust-a-try/)


## Memory model

- No explicit allocation and deallocation
- No garbage collector either
- Each value in Rust has an owner and there can only be one owner at a time.
- When the owner goes out of scope, the value is dropped.
- Rust knows the size of all stack allocations at compile time

This does not compile:
```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

- Rust's memory is managed a bit like money: one owner, it can be borrowed.
- We can borrow references: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing
- At any given time, you can have either one mutable reference or any number of immutable references.


## Installing Rust

Preferred way to install Rust for most developers: https://www.rust-lang.org/tools/install

Verify your installations of `cargo` and `rustc`:
```
$ cargo --version
cargo 1.39.0 (1c6ec66d5 2019-09-30)

$ rustc --version
rustc 1.39.0 (4560ea788 2019-11-04)
```


## Hands-on demo

In this demo we will approximate pi by generating random points and computing
their distance to origin:<sup>[1](#footnote1)</sup>

![random points](img/pi_Monte-Carlo.gif)

Tasks/discussion points:
- Show how to bootstrap a project with `cargo new`
- Step out of the newly bootstrapped project
- Clone the code (this repository)
- Browse and discuss the sources
- Compile the sources with `cargo check`
- Compare `cargo check` and `cargo build`
- Discuss `cargo run`
- Experiment with `cargo fmt`
- Run the tests with `cargo test`
- Generate optimized version with `cargo build --release`
- Generate the documentation with `cargo doc`
- Discuss `cargo publish` and https://crates.io (the Rust package registry)


## Interfacing with C, C++, Fortran, and Python

### C calling Rust and Fortran calling Rust

```
cargo build --release

gfortran -L target/release/ -lpi examples/fortran/example.f90 -o fortran-example.x
gcc -L target/release/ -lpi examples/c/example.c -o c-example.x

env LD_LIBRARY_PATH=target/release/ ./fortran-example.x
env LD_LIBRARY_PATH=target/release/ ./c-example.x
```

### Python calling Rust

- [Example using CFFI](examples/python)
- Example using [PyO3](https://github.com/PyO3/pyo3): https://github.com/robertodr/rustafarian


<a name="footnote1">1</a>: GIF from https://www.soroushjp.com/2015/02/07/go-concurrency-is-not-parallelism-real-world-lessons-with-monte-carlo-simulations/
