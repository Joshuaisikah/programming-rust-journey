

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
    match s.get(0..2) {
        Some("0b") | Some("0B") => i64::from_str_radix(&s[2..], 2).expect("Invalid binary number"), 
        Some("0x") | Some("0X") => i64::from_str_radix(&s[2..], 16).expect("Invalid hex number"),
        Some("0o") | Some("0O") => i64::from_str_radix(&s[2..], 8).expect("Invalid octal number"),
        _ => s.parse::<i64>().expect("Invalid decimal number"),
    }

}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: uncomment as you implement

    #[test]
    fn test_parse_binary() {
        assert_eq!(parse_number("0b1010"), 10);
        assert_eq!(parse_number("0B1111"), 15);
        assert_eq!(parse_number("0b0"), 0);
    }

    #[test]
    fn test_parse_hex() {
        assert_eq!(parse_number("0xFF"), 255);
        assert_eq!(parse_number("0x10"), 16);
        assert_eq!(parse_number("0XFF"), 255);
    }

    #[test]
    fn test_parse_octal() {
        assert_eq!(parse_number("0o10"), 8);
        assert_eq!(parse_number("0o77"), 63);
    }

    #[test]
    fn test_parse_decimal() {
        assert_eq!(parse_number("42"), 42);
        assert_eq!(parse_number("0"), 0);
        assert_eq!(parse_number("-5"), -5);
    }

    #[test]
    #[should_panic]
    fn test_invalid_binary() {
        parse_number("0b102"); // 2 is not a valid binary digit
    }

    #[test]
    fn placeholder() {
        assert!(true);
    }
}
