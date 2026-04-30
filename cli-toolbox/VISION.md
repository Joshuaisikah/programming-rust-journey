# CLI Toolbox — Vision

## What You're Building

A task manager that runs entirely in the terminal. You interact with it by typing commands:

```
cargo run -- add "buy milk" high
cargo run -- list
cargo run -- list --all
cargo run -- done 1
cargo run -- delete 2
cargo run -- search milk
```

You type a command, it does something, it prints a result. That is the whole app.

## The Real Point

The to-do app is not the goal — it is the vehicle. This project forces you to use
several Rust concepts *together* for the first time, in a realistic context:

- **Structs and Enums** — A `Task` is a struct with fields (id, name, priority, done).
  `Priority` is an enum: only High, Medium, or Low — nothing else is allowed.
  `Command` is also an enum that represents what the user typed as a proper Rust type.

- **Pattern Matching** — When the user types `add` or `done` or `search`, the program
  must figure out which command it is and what to do. `match` is the right tool here.

- **Error Handling** — What if the user types `done abc` instead of a number?
  What if they try to complete a task that does not exist? This project makes you
  treat failure as a first-class concern using `Result` and custom error types.

- **Module Structure** — The code is split across files with single responsibilities.
  This is how real Rust projects are organized and you are learning that pattern here.

## Architecture: How the Files Connect

```
main.rs
  │
  │  reads CLI args from the OS
  │  calls parse_command() → Command
  │  calls execute(cmd, tasks) → Result<String>
  │  prints the result or error
  │
  ├── commands.rs   ← the brain: contains parse_command() and execute()
  │     │               parse_command turns raw string args into a Command enum
  │     │               execute acts on the Command and mutates the task list
  │     │
  │     ├── models.rs    ← the data: Task struct and Priority enum live here
  │     │                   commands.rs creates Tasks and reads their fields
  │     │
  │     └── errors.rs    ← the failures: CliError enum lives here
  │                          every Result<_, CliError> in the project uses this
  │
  ├── display.rs    ← the output: print_usage() and formatting helpers
  │                    only responsible for how things look on screen
  │
  └── text.rs       ← text utilities: string helpers used by display and commands
```

Data flows in one direction: `main` → `commands` → `models`. Nothing flows backwards.
`errors` and `text` are shared utilities that any module can use without causing cycles.

## Why Each File Exists

**`main.rs`** — The entry point. Its only job is to read `env::args()`, call
`parse_command`, call `execute`, and print the result. It owns nothing except
the task list for the duration of one run. Kept thin so all real logic is testable
without running a binary.

**`models.rs`** — Defines what a `Task` is and what `Priority` means. Separated
from everything else so the data definition never gets tangled with the logic
that acts on it. If you add a new field to `Task`, you only touch this file.

**`commands.rs`** — The core logic. `parse_command` translates raw user input into
a typed `Command` value. `execute` takes that typed command and applies it to the
task list. Separated from `main.rs` so it can be tested in unit tests without
spawning a process.

**`errors.rs`** — All error variants in one place. Separated so every module can
import `CliError` without importing anything else. Avoids circular dependencies.

**`display.rs`** — All printing logic. Separated so you can change how output looks
without touching the logic in `commands.rs`. Also makes `commands.rs` testable:
tests assert on return values, not on what was printed to stdout.

**`text.rs`** — Small string utilities shared by `display` and `commands`. Separated
to avoid duplicating the same helper in two files.

## Chapters It Covers

Ch06 Expressions · Ch07 Error Handling · Ch08 Modules ·
Ch09 Structs · Ch10 Enums & Pattern Matching · Ch17 Strings

## Mental Model

Think of a vending machine:
- `models.rs` describes **what is inside** the machine
- `commands.rs` reads **which button was pressed** and acts on it
- `errors.rs` handles **when the machine cannot fulfill the request**
- `display.rs` controls **what the screen on the machine shows**
- `main.rs` is the **power switch** — it starts everything and connects the parts
