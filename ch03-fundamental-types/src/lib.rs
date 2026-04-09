// ============================================================
// Chapter 3: Fundamental Types — Library Root
// ============================================================
//
// This file declares all public modules and re-exports the
// types/functions that main.rs needs.
//
// 🎓 C# NOTE:
//   Think of lib.rs as the "namespace root" of the project.
//   pub mod = making a module public (like a public class file).
//   pub use = re-exporting so callers don't need deep paths.

pub mod parser;      // Number parsing (binary, hex, octal, decimal)
pub mod calculator;  // Arithmetic and bitwise operations
pub mod converter;   // Base and temperature conversions
pub mod display;     // Formatting and output helpers

// Re-export the main entry functions so main.rs stays clean
pub use calculator::calculate;
pub use converter::{convert_base, convert_temperature};
pub use display::{display_in_all_bases, print_usage};
pub use parser::parse_number;

