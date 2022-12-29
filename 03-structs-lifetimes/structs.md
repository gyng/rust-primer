# Structs and lifetimes

https://doc.rust-lang.org/1.9.0/book/lifetimes.html

- Lifetimes and the borrow checker are the trickiest concept in Rust for beginners
- In essence, variables can only be used by one thing at a time
  - Prevents an entire class of memory unsafety bugs

## Objectives

- Syntax
- Feel the pain
- Understand lifetime compiler errors and how to generally fix them
- Derive attribute `#[derive(Clone, Copy, Debug)]`
- Understand a bit of why Rust needs this and how it helps

## Exercise

Fix test and compile failures in `./exercise`.

- `struct`
- `struct` visibility
- `impl`
- Mutable `struct`
- Borrow checker (implicit lifetimes)
