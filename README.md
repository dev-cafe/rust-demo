

# rust-demo

Hands-on demo of the Rust programming language.


## History of Rust

(to be written)


## Why Rust

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


## Great resources

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [Why try Rust for scientific computing?](https://erambler.co.uk/blog/why-give-rust-a-try/)


## Memory model

(to be written)


## Installing Rust

(to be written)


## Hands-on demo

Cargo check/fix/test/fmt/doc/run/publish ...

(to be written and described)


## Interfacing with C, C++, Fortran, and Python

### C calling Rust and Fortran calling Rust

```
cargo build --release

gfortran -L target/release/ -lpi examples/fortran/example.f90 -o fortran-example.x
gcc -L target/release/ -lpi examples/c/example.c -o c-example.x

env LD_LIBRARY_PATH=target/release/ ./fortran-example.x
env LD_LIBRARY_PATH=target/release/ ./c-example.x
```
