---
marp: true
---

# rust-primer

## A taste of Rust

2021

$\approx$ 3–4 hours

---

```
git clone https://github.com/gyng/rust-primer.git
```

or

```
https://github.com/gyng/rust-primer/archive/refs/heads/main.zip
```

or

```
https://gitpod.io#https://github.com/gyng/rust-primer
```

or

GitHub Codespace: Click the green `Code` button at the top right > `Codespace` > Create

---

## Rust

>Rust has been voted the "most loved programming language" in the Stack Overflow Developer Survey every year since 2016.

<!-- We're here to get a high-level answer to the questions: what and why Rust? -->

---

## Me

- First line of Rust code in 2014
- Intermediate ___(!)___ Rust programmer

<!-- I learn more about programming the more I write Rust -->

---

## Rust experience

A few small and medium-sized ($\lt$ 5K LoC) Rust programs in production

<!-- I would write more in production, if I could -->

---

## Bulk CSV data sanitizer

Count, parse, clean terrabytes of CSV

- small 32-core server in an on-premise server
- 50$\times$ speed up over (somewhat optimised) Python
- 30$\times$ over PyPy

---

## Kubernetes job monitor

Export logs, metrics, and artifacts

- 14MB peak memory usage while streaming and uploading GB-sized ZIP files

---

## Prerequisites

- Internet access
- Know how to program in some programming language
- Able to `git clone`
- A machine where you can install rust
- Visual Studio Code preferred but not required

---

## Objectives

Answer the question: Why should I try Rust?

- Rust setup
- A taste of Rust programming
- Write a simple, semi-realistic program at the end

---

## Non-objectives

- Anything beyond the basics
- A good understanding of using the basics of Rust

<!-- It took my newbie programmer brain weeks before I felt comfortable with Rust. -->

---

# It's okay to not come out of this with a complete understanding of what happened!

It's about getting a first impression


---

## Format

- Topics in each subdirectory
- Intro demo for each topic
- Exercise for each topic
- Solution for each topic provided

---

## Topics

- Setup
- Basic syntax
- Structs and exposure to lifetimes
- Generics and traits
- `Option`, `Result`, enums, and pattern matching
- Strings
- External libraries
- ~~A taste of async~~
- Towards production

---

## References and links

- [The Rust Book](https://doc.rust-lang.org/book/) start here!
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Rust for C++ programmers](https://github.com/nrc/r4cppp)
- [A half hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)
- [Considering Rust](https://www.youtube.com/watch?v=DnT-LUQgc7s) ([slides](https://jon.thesquareplanet.com/slides/considering-rust/export.pdf))
- [Rustonomicon](https://doc.rust-lang.org/nomicon/) "The Dark Arts of Unsafe Rust"
- [2020 State of Rust](https://blog.rust-lang.org/2020/12/16/rust-survey-2020.html)
