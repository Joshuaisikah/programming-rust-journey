// ============================================================
// calculator.rs — Arithmetic & Bitwise Operations
// ============================================================
//
// Responsibility: perform operations on two i64 values and
// print results in all bases via display::display_in_all_bases.
//
// Supported operators:
//   Arithmetic: +  -  *  /  %
//   Bitwise:    AND/&   OR/|   XOR/^   <<   >>
//
// 🎓 C# NOTE:
//   Rust's match is exhaustive — the compiler forces you to
//   handle every case. Use _ as catch-all (like default:).
//   Integer overflow panics in debug mode (unlike C# silent wrap).
//   Use checked_add / saturating_add for safe arithmetic.

use crate::display::display_in_all_bases;

/// Performs an arithmetic or bitwise operation on two i64 values
/// and prints the result in all number bases.
///
/// # Examples
/// ```
/// use ch03_fundamental_types::calculator::calculate;
/// calculate(10, "+", 5);   // prints 15 in all bases
/// calculate(12, "AND", 10); // prints 8 in all bases
/// ```
pub fn calculate(a: i64, op: &str, b: i64) {
    // TODO: implement
    //
    // let result = match op {
    //     "+"         => a + b,
    //     "-"         => a - b,
    //     "*"         => a * b,
    //     "/"         => a / b,
    //     "%"         => a % b,
    //     "AND" | "&" => a & b,
    //     "OR"  | "|" => a | b,
    //     "XOR" | "^" => a ^ b,
    //     "<<"        => a << b,
    //     ">>"        => a >> b,
    //     _ => {
    //         eprintln!("Unknown operator: {}", op);
    //         return;
    //     }
    // };
    //
    // println!("\nResult: {}", result);
    // display_in_all_bases(result);
    todo!("implement calculate")
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    // Note: calculate() prints to stdout, so unit tests are better
    // done by extracting the computation into a helper that returns
    // the result. Consider refactoring when you implement this.

    #[test]
    fn placeholder() {
        assert!(true);
    }
}
