// ============================================================
// parser.rs — Number Parsing
// ============================================================
//
// Responsibility: parse a string into i64 supporting all bases.
//
// Supported formats:
//   "0b1010"  → binary      → 10
//   "0xFF"    → hex         → 255
//   "0o77"    → octal       → 63
//   "42"      → decimal     → 42
//
// 🎓 C# NOTE:
//   Similar to Convert.ToInt64("FF", 16) but prefix-driven.
//   i64::from_str_radix = Convert.ToInt64 with a base argument.
//   .expect() = throw new Exception(...) on parse failure.

/// Parses a number string with optional base prefix into i64.
///
/// # Examples
/// ```
/// use ch03_fundamental_types::parser::parse_number;
/// assert_eq!(parse_number("0xFF"), 255);
/// assert_eq!(parse_number("0b1010"), 10);
/// assert_eq!(parse_number("42"), 42);
/// ```
pub fn parse_number(s: &str) -> i64 {
    // TODO: implement
    //
    // if s.starts_with("0b") || s.starts_with("0B") {
    //     i64::from_str_radix(&s[2..], 2).expect("Invalid binary number")
    // } else if s.starts_with("0x") || s.starts_with("0X") {
    //     i64::from_str_radix(&s[2..], 16).expect("Invalid hex number")
    // } else if s.starts_with("0o") || s.starts_with("0O") {
    //     i64::from_str_radix(&s[2..], 8).expect("Invalid octal number")
    // } else {
    //     s.parse::<i64>().expect("Invalid decimal number")
    // }
    todo!("implement parse_number")
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: uncomment as you implement

    // #[test]
    // fn test_parse_binary() {
    //     assert_eq!(parse_number("0b1010"), 10);
    //     assert_eq!(parse_number("0B1111"), 15);
    //     assert_eq!(parse_number("0b0"), 0);
    // }

    // #[test]
    // fn test_parse_hex() {
    //     assert_eq!(parse_number("0xFF"), 255);
    //     assert_eq!(parse_number("0x10"), 16);
    //     assert_eq!(parse_number("0XFF"), 255);
    // }

    // #[test]
    // fn test_parse_octal() {
    //     assert_eq!(parse_number("0o10"), 8);
    //     assert_eq!(parse_number("0o77"), 63);
    // }

    // #[test]
    // fn test_parse_decimal() {
    //     assert_eq!(parse_number("42"), 42);
    //     assert_eq!(parse_number("0"), 0);
    //     assert_eq!(parse_number("-5"), -5);
    // }

    // #[test]
    // #[should_panic]
    // fn test_invalid_binary() {
    //     parse_number("0b102"); // 2 is not a valid binary digit
    // }

    #[test]
    fn placeholder() {
        assert!(true);
    }
}
