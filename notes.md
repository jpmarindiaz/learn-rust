
# rustup
Open documentation
`rustup doc`

Update rust to the latest
`rustup` 

# rustc

compile
`rustc main.rs`

# run
./main


# Cargo

cargo --version

New project
`cargo new hello_cargo`

Build and run a cargo project
`cargo build`

Run test executable
`./target/debug/1-3-hello_cargo`

Build and run
`cargo run`

`Cargo.lock` to keep track of dependencies

Does not produce an executable
`cargo check`

Build for release
`cargo build --release`

Update Crate versions
`cargo update`

Open documentation of the crates you use
`cargo doc --open`

## Coding

Variables are immutable by default

& -> represents a reference
References are immutable by default.

The `Result` type are enumerations. Enumetations can have a fixed set of values, called variants.

For `Result` the variants are `Ok` and `Err`. `Result` has methods, like `expect`

`use rand::Rng;` brings up the Rng Trait

Constants are always immutable.

There is a type `Wrapping` for cyclic numbers

Rust’s char type represents a Unicode Scalar Value

Compound types:
- Tuples
- Arrays

Arrays must have fixed size, allocated on stack, not heap.
Vectors are more flexible, they are allowed to grow.

# Functions

If a function is defined inside the program it can be called from inside the main function.

Rust doesn't care where you define your functions.

In function definitions you must declare the type of each parameter

# Control flows

Because `if` is an expression, it can be used on the right side o a `let` statement.

# Ownership

Stack: Fast, all data takes a known, fixed size.
Heap: Slower than stack, because you have to follow a pointer to get there.

Minimize the amount of duplicate data on the heap. Clean up unused data.

Each value in Rust has a variable that's called its owner.
There can be only one owner at a time.
When the owner goes our of scope the value will be droped.

When a variable `s` goes out of scope, Rust calls a special function called `drop` to return the memory to the OS.

Rust will never automatically create deep copies of the data.

There is a `Copy` trait. If a type has the `Copy` trait, an older variable is still usable after asignment.

Rust won't let annotate a type with `Copy` if it has the `Drop` trait.









