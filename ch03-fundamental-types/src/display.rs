// ============================================================
// display.rs — Formatting & Output Helpers
// ============================================================
//
// Responsibility: all terminal output lives here.
// Keep display logic separate from calculation logic.
//
// 🎓 C# NOTE:
//   This is like a static utility class for Console.WriteLine.
//   Format specifiers in Rust:
//     {}    → default (decimal)
//     {:X}  → uppercase hex
//     {:x}  → lowercase hex
//     {:b}  → binary
//     {:o}  → octal
//     {:08b} → binary, zero-padded to 8 digits
//     {:.2} → float with 2 decimal places

/// Prints a number in decimal, hex, binary, and octal.
///
/// # Example output
/// ```text
///   Decimal:  42
///   Hex:      0x2A
///   Binary:   0b101010
///   Octal:    0o52
/// ```
pub fn display_in_all_bases(num: i64) {
    println!("  Decimal:  {}", num);
    println!("  Hex:      0x{:X}", num);
    println!("  Binary:   0b{:b}", num);
    println!("  Octal:    0o{:o}", num);
}

/// Prints CLI usage instructions.
pub fn print_usage() {
    println!("Usage:");
    println!("  cargo run -- <num1> <op> <num2>");
    println!("  cargo run -- convert <number> <base>");
    println!("  cargo run -- temp <value> <from> <to>");
    println!();
    println!("Examples:");
    println!("  cargo run -- 10 + 5");
    println!("  cargo run -- 12 AND 10");
    println!("  cargo run -- convert 42 hex");
    println!("  cargo run -- temp 100 C F");
    println!();
    println!("Operators: + - * / % AND OR XOR << >> (& | ^)");
    println!("Bases: binary hex octal decimal (or abbreviations: bin b, hex h, oct o, dec d)");
    println!("Temperatures: C F K");
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    // Note: These functions print to stdout, so testing them directly
    // would require capturing stdout. For now, we'll keep the placeholder.
    // In a real application, you might want to return formatted strings
    // instead of printing directly for better testability.

    #[test]
    fn placeholder() {
        assert!(true);
    }
}
