# datastruct-lib — Vision

## What You're Building

A library of reusable data structures and functional utilities — the kind of thing
you would publish as a crate for other Rust programs to use. There is no user-facing
app here. The output is the library itself and its tests passing.

The structures you will build:

- **Stack\<T\>** — a generic push/pop stack that works with any type
- **Matrix** — a 2D numerical grid with `+`, `*`, and indexing via `matrix[row][col]`
- **Fibonacci iterator** — an iterator that lazily produces the Fibonacci sequence
- **RunningSum iterator** — an iterator that accumulates a running total
- **Collection utilities** — working with HashMap, BTreeMap, HashSet, VecDeque

## The Real Point

This project is about *abstraction* — writing code that works for many types at once
rather than hardcoding `i32` or `String` everywhere.

- **Traits & Generics** — You write `Stack<T>` once and it works for integers,
  strings, structs, anything. Trait bounds (`T: Clone`) let you constrain what
  is allowed inside.

- **Operator Overloading** — Rust lets you define what `+` and `*` mean for your
  own types. After implementing `Add` and `Mul` for `Matrix`, you can write
  `matrix_a + matrix_b` in natural math notation.

- **Utility Traits** — `Display` (how a type prints), `Default` (a sensible zero
  value), `Drop` (cleanup when a value is destroyed), `Clone` (explicit copying).

- **Closures** — Functions you can pass around as values. You will write utilities
  like `map`, `filter`, and `compose` that take closures as arguments.

- **Iterators** — The heart of idiomatic Rust. You will build your own iterators
  from scratch and learn how the standard library ones (`map`, `filter`, `fold`,
  `collect`) actually work under the hood.

## Architecture: How the Files Connect

```
lib.rs
  │
  │  re-exports the public API so callers write:
  │  use datastruct_lib::Stack;
  │  not: use datastruct_lib::stack::Stack;
  │
  ├── stack.rs        ← Stack<T>: generic, owns its elements
  │
  ├── matrix.rs       ← Matrix: fixed 2D grid, implements Add/Mul/Index/Display
  │                      depends on nothing else in this library
  │
  ├── iterators.rs    ← Fibonacci, RunningSum, Chunks
  │                      each is a struct that implements the Iterator trait
  │                      depends on nothing else in this library
  │
  ├── functional.rs   ← higher-order functions that accept closures
  │                      compose, apply, map_vec, filter_vec
  │                      uses iterators internally but does not re-export them
  │
  └── collections.rs  ← demonstrations and utilities for std collections
                         HashMap, BTreeMap, HashSet, VecDeque usage patterns
```

Each file is **independent** — none imports from the others. They all report to
`lib.rs`. This is intentional: data structure modules should not depend on each
other; they are parallel tools, not a chain.

## Why Each File Exists

**`lib.rs`** — The public face of the crate. Declares all modules and re-exports
the types that users of this library actually need. Kept empty of logic — it is
purely the front door.

**`stack.rs`** — Teaches generics and trait bounds in isolation. A stack has the
simplest possible interface (`push`, `pop`, `peek`, `is_empty`) so you can focus
entirely on making it work for any type `T` without distraction.

**`matrix.rs`** — The operator overloading chapter in practice. Matrix is chosen
because math on matrices is naturally expressed with `+` and `*`, so the motivation
for overloading is obvious. Also implements `Index` so `m[row][col]` works, and
`Display` so `println!("{}", m)` works.

**`iterators.rs`** — Writing a custom iterator means implementing one trait with
one method: `fn next(&mut self) -> Option<Self::Item>`. This file shows that the
entire iterator machinery — `map`, `filter`, `collect`, `zip` — works on your
custom types for free once you implement that one method.

**`functional.rs`** — Teaches how closures are used as function arguments (`Fn`,
`FnMut`, `FnOnce`). The functions here mirror what you already use from the
standard library, so you understand what is happening inside them.

**`collections.rs`** — The standard library's collections chapter applied.
`HashMap` vs `BTreeMap` (unordered/fast vs ordered), `HashSet` for deduplication,
`VecDeque` for efficient front-and-back access. Knowing which to reach for is
half the battle.

## Chapters It Covers

Ch11 Traits & Generics · Ch12 Operator Overloading ·
Ch13 Utility Traits · Ch14 Closures · Ch15 Iterators · Ch16 Collections

## Mental Model

Think of it as writing your own mini standard library. The standard library is just
a collection of well-designed structs and traits — you are learning to write things
at that same level of abstraction.
