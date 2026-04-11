// Ch22 — Unsafe Code
//
// CONCEPTS:
//   unsafe block          — opt out of borrow-checker within a delimited scope
//   *const T / *mut T     — raw pointers; no lifetime, no borrow rules
//   std::ptr::swap        — swap two values through raw pointers
//   slice::from_raw_parts — build a slice from a raw pointer + length
//   unsafe fn             — caller must uphold safety invariants
//
// GOLDEN RULE: unsafe is a promise YOU make to the compiler.
//   Wrap it in a safe abstraction; keep the unsafe surface tiny.

/// Swap two values using raw pointers.
///
/// # Safety (upheld internally)
/// Both arguments are valid, aligned, non-overlapping mutable references.
pub fn swap<T>(a: &mut T, b: &mut T) {
    todo!()
}

/// Reinterpret four consecutive bytes as a little-endian u32.
/// `bytes` must be at least 4 bytes long.
///
/// read_le_u32(&[0x01, 0x00, 0x00, 0x00]) → 1
/// read_le_u32(&[0xFF, 0x00, 0x00, 0x00]) → 255
pub fn read_le_u32(bytes: &[u8]) -> u32 {
    todo!()
}

/// Split `slice` into two halves at `mid` without bounds checking.
/// The first half is `&slice[..mid]`, the second is `&slice[mid..]`.
///
/// # Safety (upheld internally)
/// `mid` must be <= `slice.len()`.
pub fn split_at(slice: &[u8], mid: usize) -> (&[u8], &[u8]) {
    todo!()
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── swap ─────────────────────────────────────────────────

    #[test]
    #[ignore = "implement swap"]
    fn test_swap_integers() {
        let mut a = 1_i32;
        let mut b = 2_i32;
        swap(&mut a, &mut b);
        assert_eq!(a, 2);
        assert_eq!(b, 1);
    }

    #[test]
    #[ignore = "implement swap"]
    fn test_swap_strings() {
        let mut x = String::from("hello");
        let mut y = String::from("world");
        swap(&mut x, &mut y);
        assert_eq!(x, "world");
        assert_eq!(y, "hello");
    }

    #[test]
    #[ignore = "implement swap"]
    fn test_swap_vecs() {
        let mut a = vec![1, 2, 3];
        let mut b = vec![4, 5, 6];
        swap(&mut a, &mut b);
        assert_eq!(a, vec![4, 5, 6]);
        assert_eq!(b, vec![1, 2, 3]);
    }

    // ── read_le_u32 ───────────────────────────────────────────

    #[test]
    #[ignore = "implement read_le_u32"]
    fn test_read_le_u32_one() {
        assert_eq!(read_le_u32(&[0x01, 0x00, 0x00, 0x00]), 1);
    }

    #[test]
    #[ignore = "implement read_le_u32"]
    fn test_read_le_u32_256() {
        assert_eq!(read_le_u32(&[0x00, 0x01, 0x00, 0x00]), 256);
    }

    #[test]
    #[ignore = "implement read_le_u32"]
    fn test_read_le_u32_max() {
        assert_eq!(read_le_u32(&[0xFF, 0xFF, 0xFF, 0xFF]), u32::MAX);
    }

    // ── split_at ──────────────────────────────────────────────

    #[test]
    #[ignore = "implement split_at"]
    fn test_split_at_middle() {
        let data = [1u8, 2, 3, 4, 5, 6];
        let (left, right) = split_at(&data, 3);
        assert_eq!(left,  &[1, 2, 3]);
        assert_eq!(right, &[4, 5, 6]);
    }

    #[test]
    #[ignore = "implement split_at"]
    fn test_split_at_zero() {
        let data = [1u8, 2, 3];
        let (left, right) = split_at(&data, 0);
        assert!(left.is_empty());
        assert_eq!(right, &[1, 2, 3]);
    }

    #[test]
    #[ignore = "implement split_at"]
    fn test_split_at_full_length() {
        let data = [1u8, 2, 3];
        let (left, right) = split_at(&data, 3);
        assert_eq!(left, &[1, 2, 3]);
        assert!(right.is_empty());
    }
}
