// ============================================================
// cli-toolbox — Library Root
// ============================================================
//
// CHAPTERS COVERED:
//   Ch06 Expressions    — if/match as expressions, block values
//   Ch07 Error Handling — CliError, Result, ? operator
//   Ch08 Modules        — this project structure itself
//   Ch09 Structs        — Task, Config
//   Ch10 Enums/Patterns — Command, Priority, pattern matching
//   Ch17 Strings        — text utilities, formatting

pub mod errors;
pub mod models;
pub mod commands;
pub mod text;
pub mod display;
pub mod storage;

pub use errors::CliError;
pub use models::{Priority, Task};
pub use commands::{Command, execute, parse_command};
