# Capstone 01 — ripgrep Clone (rgrep)

## What You're Building

A real command-line search tool — your own version of `grep` or `ripgrep`.
You run it in the terminal and it searches files for a pattern:

```
rgrep "TODO" ./src
rgrep --ignore-case "error" ./logs
rgrep "fn " ./src --count
```

It searches recursively through directories, respects `.gitignore` files,
processes multiple files in parallel, and prints matches with color,
line numbers, and file paths — exactly like the tools professionals use every day.

## The Real Point

This is the first capstone where everything comes together into a *real tool*
that someone would actually install and use:

- **Error handling at scale** — many things can go wrong when searching files:
  permission denied, binary files, broken symlinks. You handle each gracefully
  without crashing.

- **Parallelism with Rayon** — searching 10,000 files one at a time is slow.
  Rayon lets you search them in parallel with almost no code change. You see
  firsthand how Rust's ownership model makes parallelism safe.

- **Regex** — real pattern matching beyond simple `contains()`. You use the
  `regex` crate, understand how compiled patterns work, and apply them at scale.

- **Iterators and lazy evaluation** — reading files line by line without loading
  the whole file into memory. This is how production tools handle large files.

- **CLI design with Clap** — parsing arguments like `--ignore-case`, `--count`,
  `--no-color` in a robust, user-friendly way.

## Architecture: How the Files Connect

```
main.rs
  │
  │  parses CLI args into a Config via clap
  │  calls run(config) from lib.rs
  │  prints the match count or error
  │
lib.rs
  │
  │  run() is the top-level entry point for the library
  │  it calls walker → search → output in order
  │
  ├── config.rs    ← CLI arguments become a typed Config struct
  │                   pattern, paths, flags (ignore_case, count, color)
  │                   everything the rest of the app reads from
  │
  ├── walker.rs    ← traverses the filesystem
  │                   uses walkdir + ignore crates to recurse directories
  │                   respects .gitignore files automatically
  │                   produces a list of file paths to search
  │
  ├── matcher.rs   ← compiles the regex pattern and runs it against a line
  │                   returns a Match value (line number + matched text)
  │                   isolated so the regex logic never touches file I/O
  │
  ├── search.rs    ← reads a file and applies matcher to each line
  │                   uses Rayon to search multiple files in parallel
  │                   collects results without printing them
  │
  └── output.rs    ← formats and prints results to stdout
                      handles color, line numbers, file paths
                      completely separated from search so tests can assert
                      on Match values without capturing stdout
```

Data flows in one direction: `config` is read-only input → `walker` finds files
→ `search` finds matches → `output` displays them. No module reaches backwards.

## Why Each File Exists

**`main.rs`** — Entry point only. Parses args with clap, calls `run()`, exits.
Kept minimal so the entire program is testable as a library without running a binary.

**`config.rs`** — All CLI options live here as a single `Config` struct. Every
other module reads from `Config` rather than parsing `env::args()` directly.
This means you can create a `Config` in tests without typing shell arguments.

**`walker.rs`** — Directory traversal is its own problem. The `walkdir` crate
handles recursion and the `ignore` crate handles `.gitignore`. Separating this
means `search.rs` receives a flat list of paths and never needs to think about
the filesystem structure above it.

**`matcher.rs`** — The regex is compiled *once* (expensive) and then applied to
many lines (cheap). Separating matching from searching makes this optimization
obvious and testable — you can test matching logic against strings without
creating any files.

**`search.rs`** — Reads files and applies `matcher` to each line. Rayon's
`par_iter()` goes here — one line turns sequential search into parallel search.
Separated from `output.rs` so results can be collected, sorted, or counted
before anything is printed.

**`output.rs`** — All terminal output: colors via the `colored` crate, line number
formatting, file path prefixes. Separated so you can change the display format
without touching any search logic, and so search logic is testable without
capturing stdout.

## Chapters It Combines

Ch07 Error Handling · Ch14 Closures · Ch15 Iterators · Ch17 Strings · Ch18 I/O

## Mental Model

Think of ripgrep (the fastest grep tool in existence, written in Rust). You are
building a simplified version of it from scratch. Every design decision you make —
how to walk directories, how to match lines, how to print results — is a decision
the ripgrep authors also had to make. After this capstone, you will understand
*why* ripgrep is designed the way it is.
