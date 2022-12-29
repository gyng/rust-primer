# 01 Hello, Rust

Set up Rust and your environment.

## Objectives

- Install Rust
- Use cargo
- Install tooling
- Compile a file, compile a project

## Exercise

Hello, World!

1. Install Rust if you don't already have it

   - https://rustup.rs/

2. Create a Rust project

   - Delete existing files in `./exercise`, and do a `cargo init`

3. Add useful tooling

   - `rustup component add rustfmt`
   - `rustup component add clippy`
   - `cargo fmt`
   - `cargo clippy`

4. Add IDE/editor integration

   - Check out rust-analyzer if you're using a supported IDE
     - vscode (recommended): https://rust-analyzer.github.io/manual.html#vs-code
     - emacs/vim/sublime (read instructions): https://rust-analyzer.github.io/manual.html
   - If you're an JetBrains/IntelliJ user
     - https://www.jetbrains.com/rust/
     - https://plugins.jetbrains.com/plugin/8182-rust

5. If you have time, check out the standard library documentation https://doc.rust-lang.org/std/index.html
