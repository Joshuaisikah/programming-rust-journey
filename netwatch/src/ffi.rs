// Ch23 — Foreign Functions (FFI)
//
// CONCEPTS:
//   extern "C" { }    — declare C functions callable from Rust
//   #[no_mangle]      — preserve the symbol name so C can link against it
//   extern "C" fn     — Rust function with C calling convention
//   CString           — owned null-terminated string for passing to C
//   CStr              — borrowed null-terminated string received from C
//   std::os::raw      — C-compatible primitive types (c_char, c_int, etc.)

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

extern "C" {
    /// C standard library strlen — byte count up to (not including) '\0'.
    fn strlen(s: *const c_char) -> usize;

    /// C standard library abs — absolute value of a signed integer.
    fn abs(n: c_int) -> c_int;
}

/// Safe wrapper: return the C strlen of `s`.
/// Panics if `s` contains an interior null byte.
pub fn c_strlen(s: &str) -> usize {
    todo!()
}

/// Safe wrapper: return the absolute value of `n` via the C abs function.
pub fn c_abs(n: i32) -> i32 {
    todo!()
}

/// Convert a Rust &str → CString → back to &str and return it as a String.
/// Demonstrates the round-trip required for C string interop.
pub fn cstring_round_trip(s: &str) -> String {
    todo!()
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    // ── c_strlen ──────────────────────────────────────────────

    #[test]
    #[ignore = "implement c_strlen"]
    fn test_c_strlen_matches_rust_len() {
        let s = "hello";
        assert_eq!(c_strlen(s), s.len());
    }

    #[test]
    #[ignore = "implement c_strlen"]
    fn test_c_strlen_empty() {
        assert_eq!(c_strlen(""), 0);
    }

    #[test]
    #[ignore = "implement c_strlen"]
    fn test_c_strlen_longer_string() {
        let s = "netwatch";
        assert_eq!(c_strlen(s), 8);
    }

    // ── c_abs ─────────────────────────────────────────────────

    #[test]
    #[ignore = "implement c_abs"]
    fn test_c_abs_positive_unchanged() {
        assert_eq!(c_abs(42), 42);
    }

    #[test]
    #[ignore = "implement c_abs"]
    fn test_c_abs_negative_becomes_positive() {
        assert_eq!(c_abs(-7), 7);
    }

    #[test]
    #[ignore = "implement c_abs"]
    fn test_c_abs_zero() {
        assert_eq!(c_abs(0), 0);
    }

    // ── cstring_round_trip ────────────────────────────────────

    #[test]
    #[ignore = "implement cstring_round_trip"]
    fn test_round_trip_preserves_content() {
        assert_eq!(cstring_round_trip("hello"), "hello");
    }

    #[test]
    #[ignore = "implement cstring_round_trip"]
    fn test_round_trip_empty_string() {
        assert_eq!(cstring_round_trip(""), "");
    }
}
