// Ch21 — Macros
//
// CONCEPTS:
//   macro_rules!        — declarative macro; match token patterns, emit code
//   $x:expr             — capture an expression fragment
//   $($x:expr),*        — capture zero or more comma-separated expressions
//   $(,)?               — allow an optional trailing comma
//   stringify!($e)      — turn an expression into its source-code string
//   $name:ident         — capture an identifier fragment
//   $inner:ty           — capture a type fragment
//
// NOTE ON STUBS:
//   Macros can't use todo!() the way functions can.
//   Each macro below has a stub body — your job is to REPLACE it with
//   the correct token expansion. The tests define exactly what it must produce.

/// Build a Vec from a comma-separated list of expressions.
/// collect_vec![1, 2, 3]   → vec![1, 2, 3]
/// collect_vec![]          → vec![]
#[macro_export]
macro_rules! collect_vec {
    ($($x:expr),* $(,)?) => {
        unimplemented!("replace with: vec![$($x),*]")
    };
}

/// Assert equality; on failure print both values and their source text.
/// assert_eq_verbose!(1 + 1, 2)  — passes silently
/// assert_eq_verbose!(1, 2)      — panics with a message showing source + values
#[macro_export]
macro_rules! assert_eq_verbose {
    ($left:expr, $right:expr) => {
        unimplemented!("evaluate both sides; compare with !=; panic via stringify! on mismatch")
    };
}

/// Generate a newtype struct and a `new` constructor.
/// newtype!(Meters, f64)  expands to:
///   pub struct Meters(pub f64);
///   impl Meters { pub fn new(val: f64) -> Self { Meters(val) } }
#[macro_export]
macro_rules! newtype {
    ($name:ident, $inner:ty) => {
        unimplemented!("emit struct $name($inner) and an impl block with fn new")
    };
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {

    // ── collect_vec! ──────────────────────────────────────────

    #[test]
    #[ignore = "implement collect_vec!"]
    fn test_collect_vec_basic() {
        let v: Vec<i32> = collect_vec![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    #[ignore = "implement collect_vec!"]
    fn test_collect_vec_trailing_comma() {
        let v: Vec<i32> = collect_vec![10, 20, 30,];
        assert_eq!(v.len(), 3);
    }

    #[test]
    #[ignore = "implement collect_vec!"]
    fn test_collect_vec_empty() {
        let v: Vec<i32> = collect_vec![];
        assert!(v.is_empty());
    }

    #[test]
    #[ignore = "implement collect_vec!"]
    fn test_collect_vec_strings() {
        let v: Vec<&str> = collect_vec!["a", "b", "c"];
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    // ── assert_eq_verbose! ────────────────────────────────────

    #[test]
    #[ignore = "implement assert_eq_verbose!"]
    fn test_assert_eq_verbose_passes_on_equal() {
        assert_eq_verbose!(1 + 1, 2);
        assert_eq_verbose!("hello", "hello");
    }

    // ── newtype! ──────────────────────────────────────────────
    // These tests are left as compile-time checks only.
    // When newtype! is implemented, uncomment the body.

    #[test]
    #[ignore = "implement newtype! — it must emit a struct definition and impl block"]
    fn test_newtype_constructor() {
        // newtype!(Meters, f64);
        // let m = Meters::new(1.5);
        // assert_eq!(m.0, 1.5);
    }

    #[test]
    #[ignore = "implement newtype! — it must work for multiple different types"]
    fn test_newtype_two_different_types() {
        // newtype!(Kilograms, f64);
        // newtype!(Seconds, u32);
        // let kg = Kilograms::new(70.0);
        // let s  = Seconds::new(60);
        // assert_eq!(kg.0, 70.0);
        // assert_eq!(s.0, 60);
    }
}
