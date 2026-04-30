# Forge — Vision

## What You're Building

A collection of working demonstrations across four advanced Rust topics:
strings and text processing, file I/O, multi-threaded concurrency, and async.

There is no single app here. Each module is a self-contained playground where
you implement real utilities and observe exactly how they behave.

## The Real Point

By this point in the journey you know how to model data and handle errors.
This project is about **how Rust programs interact with the world**:

- **Strings** are harder than they look. Rust has `String`, `&str`, `OsString`,
  `Path`, `bytes` — each exists for a reason. You will understand when to use which.

- **I/O** in Rust is explicit and buffered by default. You learn to think about
  reads and writes as fallible operations that consume resources.

- **Concurrency** — Rust's ownership system prevents data races *at compile time*.
  This project shows you exactly how: why you need `Arc<Mutex<T>>` to share data
  across threads, and how channels let threads communicate without sharing.

- **Async** is a different model from threads — instead of blocking, tasks yield
  and let other tasks run. You learn when async is better than threads and when it is not.

## Architecture: How the Files Connect

```
main.rs
  │
  │  orchestrates demonstrations — calls into each module
  │  and prints what happens. Entry point only, contains no logic.
  │
  ├── strings.rs     ← self-contained: no imports from other modules
  │                     Unicode, &str vs String, formatting, parsing, splitting
  │
  ├── io.rs          ← self-contained: uses std::fs, std::io, std::path
  │                     reading files line by line, writing, buffered I/O,
  │                     walking directories, working with Path/PathBuf
  │
  ├── concurrency.rs ← self-contained: uses std::thread, Mutex, Arc, channels
  │                     spawning threads, sharing state, message passing,
  │                     the difference between Arc (shared ownership) and
  │                     Mutex (exclusive access)
  │
  └── async_ops.rs   ← self-contained: uses tokio
                        async fn, .await, tokio::spawn, joining tasks,
                        the difference between async tasks and OS threads
```

The four modules do **not** import from each other. Each teaches a completely
different system and mixing them would obscure which feature causes which behavior.
`main.rs` calls into each one to show them working, but they are independent.

## Why Each File Exists

**`main.rs`** — The runner. Its only job is to call demo functions from each module
and print the output. Kept empty of logic so the real teaching happens inside the
modules, not here.

**`strings.rs`** — Strings are the first thing that surprises developers new to Rust.
The distinction between `String` (owned heap data) and `&str` (borrowed view into
bytes) is fundamental to ownership. This module isolates that complexity and forces
you to work through it without also worrying about threads or I/O.

**`io.rs`** — Every real program reads or writes something. I/O in Rust is always
fallible (`Result`) and always explicit about buffering. This module is where you
learn to work with the filesystem the Rust way — no implicit assumptions, no hidden
exceptions.

**`concurrency.rs`** — Threads are where ownership gets tested hardest. Rust prevents
data races by enforcing that data either has one owner at a time or is behind a lock.
This module makes you feel that enforcement directly: the compiler will reject unsafe
sharing and force you to use `Arc<Mutex<T>>` or channels.

**`async_ops.rs`** — Async is not just "faster threads." It is a different execution
model suited for programs that spend most of their time waiting (network I/O, disk).
This module exists separately because async requires a runtime (`tokio`) and a
different way of thinking about control flow. Mixing it with `concurrency.rs` would
make both harder to understand.

## Chapters It Covers

Ch17 Strings & Text · Ch18 Input & Output ·
Ch19 Concurrency · Ch20 Async

## Mental Model

A chef learning knife skills, heat control, seasoning, and timing — each in a
separate session before combining them in a full dish. Each module isolates one
hard topic so you understand it completely before the capstones mix everything together.
