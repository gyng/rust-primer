# Strings

Strings were a confusing point for me when I started learning Rust without any systems-language experience.

Remember that Rust is an abstraction layer over system memory, and that strings are _not_ a bag of bytes!

Generally, there are two types of strings in Rust you'll encounter

`&str` (slice) and `String`

Think of

- `&str` as a fixed-length array of bytes

  - Like a C++ `char*`, but UTF-8

- `String` as a variable-length array of bytes (usually UTF-8)
  - Has a underlying buffer, owns the buffer
  - Like a C++ `std::String`

Or, in simpler terms

- `&str`: valid utf-8 bytes (stack, can't grow in size)
- `String`: valid utf-8 bytes (heap, can grow in size)

## Why?

- Performance
- C interop
- Heap vs stack

<!-- You think you know strings as a programmer, but do you really? -->

## Objectives

- Create strings
- Move strings around
- Convert strings

## Exercise

Strings are everywhere in code.

Get familiar with manipulating strings, storing them in structs, and tracking which variables owns what strings at any time.

This exercise will take a bit longer as there are more things to explore.
