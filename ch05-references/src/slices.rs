// ============================================================
// slices.rs — Slices (&str, &[T])
// ============================================================
//
// KEY CONCEPTS:
//
//   A slice is a reference to a contiguous sequence — no ownership.
//
//   &str   — string slice: a reference into a String or string literal.
//   &[T]   — slice of any array or Vec<T>.
//   &mut [T] — mutable slice.
//
//   Slices carry both a pointer AND a length (fat pointer).
//   String literals are &'static str — slices into program memory.
//
//   Common slice methods:
//     .len(), .is_empty(), .first(), .last()
//     .split_at(n), .chunks(n), .windows(n)
//     .iter(), .iter_mut()
//     .contains(&x), .starts_with(&[..])

use crate::display::section;

/// Returns the first word in a string (up to the first space).
pub fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None    => s,
    }
}

/// Returns the sum of a slice of integers.
pub fn sum_slice(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

/// Demonstrates string slices and array slices.
pub fn demo_slices() {
    section("SLICES");

    // TODO: demo 1 — string slice from a String
    //   let s = String::from("hello world");
    //   let hello = &s[0..5];
    //   let world = &s[6..];
    //   println!("{} {}", hello, world);

    // TODO: demo 2 — first_word
    //   let sentence = String::from("hello world");
    //   let word = first_word(&sentence);
    //   println!("first word: {}", word);

    // TODO: demo 3 — array slice
    //   let a = [1, 2, 3, 4, 5];
    //   let mid = &a[1..4];  // [2, 3, 4]
    //   println!("{:?}", mid);

    // TODO: demo 4 — sum_slice (works for Vec and arrays via coercion)
    //   let v = vec![10, 20, 30];
    //   println!("sum = {}", sum_slice(&v));

    println!("(implement the slice demos above)");
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word_with_space() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    fn test_first_word_no_space() {
        assert_eq!(first_word("rust"), "rust");
    }

    #[test]
    fn test_first_word_empty() {
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_sum_slice_basic() {
        assert_eq!(sum_slice(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_sum_slice_empty() {
        assert_eq!(sum_slice(&[]), 0);
    }

    #[test]
    fn test_sum_slice_from_vec() {
        let v = vec![10, 20, 30];
        assert_eq!(sum_slice(&v), 60);
    }

    #[test]
    fn test_string_slice_range() {
        let s = String::from("hello world");
        assert_eq!(&s[0..5], "hello");
        assert_eq!(&s[6..], "world");
    }

    #[test]
    fn test_array_slice() {
        let a = [1, 2, 3, 4, 5];
        let mid = &a[1..4];
        assert_eq!(mid, &[2, 3, 4]);
    }

    // TODO: add tests for chunks, windows, split_at, etc.

    #[test]
    fn placeholder() {
        assert!(true);
    }
}
