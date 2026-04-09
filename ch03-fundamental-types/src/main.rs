// ============================================================
// Chapter 3: Fundamental Types — Programmer's Calculator
// ============================================================
//
// PROJECT GOAL:
//   Build a CLI calculator that handles multiple number bases,
//   bitwise operations, and unit conversions.
//
// WHAT YOU WILL LEARN:
//   - Integer types: i8, u8, i32, u32, i64, u64, usize
//   - Float types:   f32, f64 and precision trade-offs
//   - Number bases:  binary (0b), octal (0o), hex (0x), decimal
//   - Bitwise ops:   &, |, ^, <<, >>
//   - Type casting:  as, from(), try_from()
//   - Formatting:    {:X}, {:b}, {:o}, {:.2}
//
// FILE STRUCTURE (what to build):
// ─────────────────────────────────────────────────────────────
//   src/
//   └── main.rs          ← you are here (entry point + all logic)
//
//   Cargo.toml           ← already configured, no changes needed
//   PROJECT.md           ← full guide with examples, read it!
// ─────────────────────────────────────────────────────────────
//
// HOW TO RUN:
//   cargo run -- 0xFF + 42
//   cargo run -- 0b1010 AND 0b1111
//   cargo run -- convert 255 binary
//   cargo run -- temp 100 C F
//   cargo test
//
// 🎓 C# NOTE:
//   Rust has no implicit type conversions — you must be explicit.
//   Where C# lets you do: int x = myByte; (implicit widening)
//   Rust requires:        let x: i32 = my_u8 as i32;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO: Check args.len() — if less than 2, call print_usage() and return
    //
    // Match on args[1].as_str() to dispatch:
    //   "convert" → call convert_base(num, base)
    //   "temp"    → call convert_temperature(value, from, to)
    //   _         → default arithmetic: parse num1, op, num2 → call calculate()

    println!("Chapter 3: Programmer's Calculator — not yet implemented");
    print_usage();
}

// ─────────────────────────────────────────────────────────────
// SECTION 1: Number Parsing
// ─────────────────────────────────────────────────────────────
//
// Parse a string into i64, supporting multiple bases:
//   "0b1010"  → binary  → 10
//   "0xFF"    → hex     → 255
//   "0o77"    → octal   → 63
//   "42"      → decimal → 42
//
// Use i64::from_str_radix(&s[2..], radix) for prefixed bases.
// Use s.parse::<i64>() for decimal.
//
// 🎓 C# NOTE:
//   Similar to Convert.ToInt64("FF", 16) in C#
//   but Rust auto-detects the prefix instead.

fn parse_number(s: &str) -> i64 {
    // TODO: implement
    //
    // if s.starts_with("0b") || s.starts_with("0B") → radix 2
    // if s.starts_with("0x") || s.starts_with("0X") → radix 16
    // if s.starts_with("0o") || s.starts_with("0O") → radix 8
    // else → s.parse::<i64>().expect("Invalid decimal number")
    todo!("implement parse_number")
}

// ─────────────────────────────────────────────────────────────
// SECTION 2: Arithmetic + Bitwise Operations
// ─────────────────────────────────────────────────────────────
//
// Match on the operator string and compute result.
//
// Arithmetic:  +  -  *  /  %
// Bitwise:     AND/& ,  OR/|,  XOR/^,  <<,  >>
//
// After computing, print the result then call display_in_all_bases().
//
// 🎓 C# NOTE:
//   Rust's match is like a switch but exhaustive — every case
//   must be handled or the code won't compile. Use _ as catch-all.

fn calculate(a: i64, op: &str, b: i64) {
    // TODO: implement
    //
    // let result = match op {
    //     "+" => a + b,
    //     ...bitwise...
    //     _ => { eprintln!(...); return; }
    // };
    //
    // println!("Result: {}", result);
    // display_in_all_bases(result);
    todo!("implement calculate")
}

// ─────────────────────────────────────────────────────────────
// SECTION 3: Display in All Bases
// ─────────────────────────────────────────────────────────────
//
// Print the same number in decimal, hex, binary, and octal.
//
// Format specifiers:
//   {:}   → decimal (default)
//   {:X}  → uppercase hexadecimal
//   {:b}  → binary
//   {:o}  → octal
//   {:08b} → binary padded to 8 digits

fn display_in_all_bases(num: i64) {
    // TODO: implement
    //
    // println!("  Decimal: {}", num);
    // println!("  Hex:     0x{:X}", num);
    // println!("  Binary:  0b{:b}", num);
    // println!("  Octal:   0o{:o}", num);
    todo!("implement display_in_all_bases")
}

// ─────────────────────────────────────────────────────────────
// SECTION 4: Base Conversion
// ─────────────────────────────────────────────────────────────
//
// Convert a number to a specific base by name.
// Accept abbreviations: "binary"/"bin"/"b", "hex"/"h", etc.
//
// Use .to_lowercase() so "HEX" and "hex" both work.

fn convert_base(num: i64, to_base: &str) {
    // TODO: implement
    //
    // match to_base.to_lowercase().as_str() {
    //     "binary" | "bin" | "b" => println!("0b{:b}", num),
    //     ...
    // }
    todo!("implement convert_base")
}

// ─────────────────────────────────────────────────────────────
// SECTION 5: Temperature Conversion
// ─────────────────────────────────────────────────────────────
//
// Convert between Celsius (C), Fahrenheit (F), and Kelvin (K).
//
// Strategy: convert input → Celsius first, then Celsius → output.
//
// Use f64 for precision. Float literals need the .0 suffix: 32.0
// Use {:.2} to print 2 decimal places.
//
// 🎓 C# NOTE:
//   Rust's f64 = C#'s double. Rust's f32 = C#'s float.
//   Prefer f64 unless memory is critical.
//
// GOTCHA: floats are not exact!
//   0.1 + 0.2 != 0.3 in most languages including Rust.
//   Use (a - b).abs() < 0.00001 for approximate comparisons.

fn convert_temperature(value: f64, from: &str, to: &str) {
    // TODO: implement
    //
    // Step 1 — convert to Celsius:
    //   "C" → value
    //   "F" → (value - 32.0) * 5.0 / 9.0
    //   "K" → value - 273.15
    //
    // Step 2 — convert Celsius to target:
    //   "C" → celsius
    //   "F" → (celsius * 9.0 / 5.0) + 32.0
    //   "K" → celsius + 273.15
    //
    // println!("{:.2}°{} = {:.2}°{}", value, from, result, to);
    todo!("implement convert_temperature")
}

// ─────────────────────────────────────────────────────────────
// SECTION 6: Usage Help
// ─────────────────────────────────────────────────────────────

fn print_usage() {
    println!("\nUsage:");
    println!("  cargo run -- <num1> <op> <num2>       Arithmetic/bitwise");
    println!("  cargo run -- convert <number> <base>  Base conversion");
    println!("  cargo run -- temp <value> <from> <to> Temperature conversion");
    println!("\nOperators:  +  -  *  /  %  AND  OR  XOR  <<  >>");
    println!("Bases:      binary  hex  octal  decimal");
    println!("Temp units: C  F  K");
    println!("\nExamples:");
    println!("  cargo run -- 0xFF + 42");
    println!("  cargo run -- 0b1010 AND 0b1111");
    println!("  cargo run -- convert 255 binary");
    println!("  cargo run -- temp 100 C F");
}

// ─────────────────────────────────────────────────────────────
// SECTION 7: Tests
// ─────────────────────────────────────────────────────────────
//
// Uncomment and fill in as you implement each function.
// Run with: cargo test
//
// 🎓 C# NOTE:
//   #[cfg(test)] means this block only compiles during testing.
//   assert_eq!(actual, expected) is like Assert.AreEqual in NUnit/xUnit.
//   #[should_panic] marks tests that expect a panic (like Assert.Throws).

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: uncomment when parse_number is implemented
    //
    // #[test]
    // fn test_parse_binary() {
    //     assert_eq!(parse_number("0b1010"), 10);
    //     assert_eq!(parse_number("0b1111"), 15);
    // }
    //
    // #[test]
    // fn test_parse_hex() {
    //     assert_eq!(parse_number("0xFF"), 255);
    //     assert_eq!(parse_number("0x10"), 16);
    // }
    //
    // #[test]
    // fn test_parse_octal() {
    //     assert_eq!(parse_number("0o10"), 8);
    //     assert_eq!(parse_number("0o77"), 63);
    // }
    //
    // #[test]
    // fn test_parse_decimal() {
    //     assert_eq!(parse_number("42"), 42);
    //     assert_eq!(parse_number("0"), 0);
    // }
    //
    // #[test]
    // #[should_panic]
    // fn test_invalid_binary() {
    //     parse_number("0b102"); // 2 is invalid in binary
    // }

    #[test]
    fn placeholder() {
        // Remove this once you add real tests
        assert!(true);
    }
}
