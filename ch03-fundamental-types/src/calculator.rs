use crate::display::display_in_all_bases;

/// Helper function that performs the calculation without printing.
/// Returns the result or None for invalid operators.
fn compute(a: i64, op: &str, b: i64) -> Option<i64> {
    match op {
        "+" => Some(a + b),
        "-" => Some(a - b),
        "*" => Some(a * b),
        "/" => Some(a / b),
        "%" => Some(a % b),
        "AND" | "&" => Some(a & b),
        "OR" | "|" => Some(a | b),
        "XOR" | "^" => Some(a ^ b),
        "<<" => Some(a << b),
        ">>" => Some(a >> b),
        _ => None,
    }
}

/// Performs an arithmetic or bitwise operation on two i64 values
/// and prints the result in all number bases.
pub fn calculate(a: i64, op: &str, b: i64) {
    match compute(a, op, b) {
        Some(result) => {
            println!("\nResult: {}", result);
            display_in_all_bases(result);
        }
        None => {
            eprintln!("Unknown operator: {}", op);
        }
    }
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_addition() {
        assert_eq!(compute(10, "+", 5), Some(15));
        assert_eq!(compute(-5, "+", 10), Some(5));
        assert_eq!(compute(0, "+", 0), Some(0));
    }

    #[test]
    fn test_arithmetic_subtraction() {
        assert_eq!(compute(10, "-", 5), Some(5));
        assert_eq!(compute(5, "-", 10), Some(-5));
        assert_eq!(compute(0, "-", 0), Some(0));
    }

    #[test]
    fn test_arithmetic_multiplication() {
        assert_eq!(compute(10, "*", 5), Some(50));
        assert_eq!(compute(-5, "*", 10), Some(-50));
        assert_eq!(compute(0, "*", 100), Some(0));
    }

    #[test]
    fn test_arithmetic_division() {
        assert_eq!(compute(10, "/", 5), Some(2));
        assert_eq!(compute(15, "/", 4), Some(3)); // Integer division truncates
        assert_eq!(compute(-10, "/", 5), Some(-2));
    }

    #[test]
    fn test_arithmetic_modulo() {
        assert_eq!(compute(10, "%", 3), Some(1));
        assert_eq!(compute(15, "%", 4), Some(3));
        assert_eq!(compute(-10, "%", 3), Some(-1)); // In Rust, a % b has same sign as a
    }

    #[test]
    fn test_bitwise_and() {
        assert_eq!(compute(12, "AND", 10), Some(8));  // 12 & 10 = 8 (1100 & 1010 = 1000)
        assert_eq!(compute(12, "&", 10), Some(8));    // Alternative syntax
        assert_eq!(compute(255, "&", 15), Some(15));  // 11111111 & 00001111 = 00001111
    }

    #[test]
    fn test_bitwise_or() {
        assert_eq!(compute(12, "OR", 10), Some(14));  // 12 | 10 = 14 (1100 | 1010 = 1110)
        assert_eq!(compute(12, "|", 10), Some(14));   // Alternative syntax
        assert_eq!(compute(240, "|", 15), Some(255)); // 11110000 | 00001111 = 11111111
    }

    #[test]
    fn test_bitwise_xor() {
        assert_eq!(compute(12, "XOR", 10), Some(6));  // 12 ^ 10 = 6 (1100 ^ 1010 = 0110)
        assert_eq!(compute(12, "^", 10), Some(6));    // Alternative syntax
        assert_eq!(compute(255, "^", 255), Some(0));  // 11111111 ^ 11111111 = 00000000
    }

    #[test]
    fn test_bitwise_shift_left() {
        assert_eq!(compute(1, "<<", 3), Some(8));     // 1 << 3 = 8
        assert_eq!(compute(8, "<<", 2), Some(32));    // 8 << 2 = 32
        assert_eq!(compute(255, "<<", 0), Some(255)); // No shift
    }

    #[test]
    fn test_bitwise_shift_right() {
        assert_eq!(compute(8, ">>", 3), Some(1));     // 8 >> 3 = 1
        assert_eq!(compute(32, ">>", 2), Some(8));    // 32 >> 2 = 8
        assert_eq!(compute(255, ">>", 0), Some(255)); // No shift
    }

    #[test]
    fn test_invalid_operator() {
        assert_eq!(compute(10, "invalid", 5), None);
        assert_eq!(compute(10, "**", 5), None); // Exponentiation not supported
    }

    #[test]
    fn test_edge_cases() {
        // Division by zero should panic (expected behavior)
        // We don't test this as it would cause the test to fail

        // Large numbers
        assert_eq!(compute(i64::MAX, "+", 0), Some(i64::MAX));
        assert_eq!(compute(i64::MIN, "+", 0), Some(i64::MIN));

        // Negative shifts (undefined behavior in Rust, but let's test small positive)
        assert_eq!(compute(-8, ">>", 2), Some(-2)); // Arithmetic right shift
    }
}
