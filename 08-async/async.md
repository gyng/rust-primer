# Async programming in Rust

If you're familiar with Python/JS async or futures, you're mostly set.

The only additional thing Rust makes you hyper-aware is memory, and how you share memory across threads

The Rust compiler makes sure that this doesn't happen:

- two or more threads concurrently accessing a location of memory
- one or more of them is a write
- one or more of them is unsynchronised

But! Rust does not prevent logic bugs and deadlocks.

Async programming is very common in web programming: we're doing this in preparation for the next bit!

## Objectives

- Basic threading primitives: mutex, semaphore
- Sharing data across threads
- Runtimes
- Async and await
- Futures
