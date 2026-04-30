# Netwatch — Vision

## What You're Building

A network monitoring tool that demonstrates Rust's three most advanced and
most powerful features: macros, unsafe code, and FFI (Foreign Function Interface —
calling C code from Rust).

The tool collects system and network statistics. The *what it does* is secondary
to the *how it does it*, which is the point of this project.

## The Real Point

This project teaches you the edges of Rust — the parts that exist because sometimes
the safe, ergonomic path is not enough:

- **Macros** are code that writes code. Rust has two kinds: `macro_rules!`
  (pattern-matching on syntax) and procedural macros (plugins that transform
  the AST). You will learn why they exist and how to write them without abusing them.

- **Unsafe** does not mean "broken" — it means "the programmer takes responsibility
  for invariants the compiler cannot verify." Every `unsafe` block should have a
  comment explaining *why* it is safe. This project teaches you to think at that level.

- **FFI** is how Rust talks to the C ecosystem — decades of system libraries,
  OS APIs, and hardware drivers. You write `extern "C"` declarations, manage
  raw pointers across language boundaries, and understand why this requires `unsafe`.

## Architecture: How the Files Connect

```
main.rs
  │
  │  wires the three modules together and runs the tool
  │  uses macros from macros.rs to reduce boilerplate
  │  calls unsafe_ops for low-level system data
  │  calls ffi for data that requires C library calls
  │
  ├── macros.rs      ← defines custom macros used by the other modules
  │                     loaded first via #[macro_use] so macros are available
  │                     everywhere in the crate before any other module compiles
  │
  ├── unsafe_ops.rs  ← raw pointer operations and direct memory reads
  │                     every unsafe block is wrapped in a safe public function
  │                     the unsafe is contained here so the rest of the crate
  │                     never needs to think about it
  │
  └── ffi.rs         ← extern "C" declarations for C system library functions
                        converts C types (raw pointers, c_int, c_char) into
                        safe Rust types before returning them to the rest of
                        the crate. The unsafe boundary stops at this file.
```

The key architectural decision: **unsafe is pushed to the edges**. `ffi.rs` and
`unsafe_ops.rs` are the only files with `unsafe` blocks. They expose safe public
functions to `main.rs`. This is how production Rust code handles unsafe — isolate
it, wrap it, and keep the rest of the codebase clean.

## Why Each File Exists

**`main.rs`** — Orchestrator. Calls into the three modules and presents the
collected data. Uses the macros defined in `macros.rs` to avoid repeating
display boilerplate. Contains no `unsafe` itself — proof that unsafe can be
fully contained.

**`macros.rs`** — Macros must be defined before they are used, and `#[macro_use]`
makes them available crate-wide. Separated into its own file so macro definitions
do not clutter the logic in other files. This is the standard pattern for
non-trivial macros.

**`unsafe_ops.rs`** — Groups all direct memory operations in one place. When you
later audit the codebase for safety, you search one file. The public functions
in this module have safe signatures — callers do not need to know that raw
pointers are involved internally.

**`ffi.rs`** — C libraries speak a different language: raw pointers, C strings
(`*const c_char`), integer error codes instead of `Result`. This file is the
translation layer. It speaks C on one side and Rust on the other. Keeping it
separate means the ugliness of C interop never leaks into the rest of the code.

## Chapters It Covers

Ch21 Macros · Ch22 Unsafe Code · Ch23 Foreign Functions

## Mental Model

Most Rust code never needs these features. But when you are at the system level —
talking to the OS, calling a C library, eliminating the last nanosecond of overhead —
these are the tools. This project teaches you *why they exist* and *how to use them
responsibly*, so you are not afraid of them and do not misuse them.
