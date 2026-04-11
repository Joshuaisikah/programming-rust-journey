// Ch17 — Strings & Text
//
// CONCEPTS:
//   &str / String       — borrowed slice vs owned heap string
//   chars()             — Unicode scalar value iteration (not bytes)
//   split_whitespace    — splits on any whitespace, skips empty tokens
//   HashMap<String, _>  — counting and grouping by string key
//   char arithmetic     — casting char ↔ u8 for cipher shifts

use std::collections::HashMap;

/// Count how many times each whitespace-delimited word appears.
/// word_frequency("the cat sat the cat") → {"the":2, "cat":2, "sat":1}
pub fn word_frequency(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// Return true if `s` reads the same forwards and backwards.
/// Operates on Unicode chars, not bytes. Ignores nothing — exact match.
/// is_palindrome("racecar") → true
/// is_palindrome("hello")   → false
pub fn is_palindrome(s: &str) -> bool {
    todo!()
}

/// Shift every ASCII letter in `text` forward by `shift` positions, wrapping
/// at 'z'/'Z'. Non-letter characters are left unchanged.
/// caesar_cipher("abc", 1) → "bcd"
/// caesar_cipher("xyz", 3) → "abc"
pub fn caesar_cipher(text: &str, shift: u8) -> String {
    todo!()
}

/// Return the longest string that is a prefix of every word in `words`.
/// Returns "" if `words` is empty or there is no common prefix.
/// longest_common_prefix(&["flower","flow","flight"]) → "fl"
pub fn longest_common_prefix(words: &[&str]) -> String {
    todo!()
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── word_frequency ────────────────────────────────────────

    #[test]
    #[ignore = "implement word_frequency"]
    fn test_word_frequency_counts_correctly() {
        let freq = word_frequency("the cat sat the cat");
        assert_eq!(freq["the"], 2);
        assert_eq!(freq["cat"], 2);
        assert_eq!(freq["sat"], 1);
    }

    #[test]
    #[ignore = "implement word_frequency"]
    fn test_word_frequency_empty_string() {
        assert!(word_frequency("").is_empty());
    }

    #[test]
    #[ignore = "implement word_frequency"]
    fn test_word_frequency_extra_whitespace() {
        let freq = word_frequency("  hello   hello  ");
        assert_eq!(freq["hello"], 2);
        assert_eq!(freq.len(), 1);
    }

    // ── is_palindrome ─────────────────────────────────────────

    #[test]
    #[ignore = "implement is_palindrome"]
    fn test_palindrome_true() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("level"));
        assert!(is_palindrome("a"));
    }

    #[test]
    #[ignore = "implement is_palindrome"]
    fn test_palindrome_false() {
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("rust"));
    }

    #[test]
    #[ignore = "implement is_palindrome"]
    fn test_palindrome_empty_is_true() {
        assert!(is_palindrome(""));
    }

    #[test]
    #[ignore = "implement is_palindrome"]
    fn test_palindrome_unicode() {
        assert!(is_palindrome("able was i ere i saw elba"));
    }

    // ── caesar_cipher ─────────────────────────────────────────

    #[test]
    #[ignore = "implement caesar_cipher"]
    fn test_caesar_shift_one() {
        assert_eq!(caesar_cipher("abc", 1), "bcd");
    }

    #[test]
    #[ignore = "implement caesar_cipher"]
    fn test_caesar_wraps_around() {
        assert_eq!(caesar_cipher("xyz", 3), "abc");
        assert_eq!(caesar_cipher("XYZ", 3), "ABC");
    }

    #[test]
    #[ignore = "implement caesar_cipher"]
    fn test_caesar_non_alpha_unchanged() {
        assert_eq!(caesar_cipher("hello, world!", 0), "hello, world!");
    }

    #[test]
    #[ignore = "implement caesar_cipher"]
    fn test_caesar_round_trip() {
        let original = "Hello, World!";
        let encrypted = caesar_cipher(original, 13);
        let decrypted = caesar_cipher(&encrypted, 13);
        assert_eq!(decrypted, original);
    }

    // ── longest_common_prefix ─────────────────────────────────

    #[test]
    #[ignore = "implement longest_common_prefix"]
    fn test_lcp_basic() {
        assert_eq!(longest_common_prefix(&["flower", "flow", "flight"]), "fl");
    }

    #[test]
    #[ignore = "implement longest_common_prefix"]
    fn test_lcp_no_common_prefix() {
        assert_eq!(longest_common_prefix(&["dog", "car", "race"]), "");
    }

    #[test]
    #[ignore = "implement longest_common_prefix"]
    fn test_lcp_single_word() {
        assert_eq!(longest_common_prefix(&["only"]), "only");
    }

    #[test]
    #[ignore = "implement longest_common_prefix"]
    fn test_lcp_empty_slice() {
        assert_eq!(longest_common_prefix(&[]), "");
    }

    #[test]
    #[ignore = "implement longest_common_prefix"]
    fn test_lcp_full_match() {
        assert_eq!(longest_common_prefix(&["rust", "rust", "rust"]), "rust");
    }
}
