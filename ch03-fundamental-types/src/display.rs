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
    // TODO: implement
    //
    // println!("  Decimal:  {}", num);
    // println!("  Hex:      0x{:X}", num);
    // println!("  Binary:   0b{:b}", num);
    // println!("  Octal:    0o{:o}", num);
    todo!("implement display_in_all_bases")
}

/// Prints CLI usage instructions.
pub fn print_usage() {
    // TODO: implement
    //
    // println!("Usage:");
    // println!("  cargo run -- <num1> <op> <num2>");
    // println!("  cargo run -- convert <number> <base>");
    // println!("  cargo run -- temp <value> <from> <to>");
    todo!("implement print_usage")
}

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        assert!(true);
    }
}
