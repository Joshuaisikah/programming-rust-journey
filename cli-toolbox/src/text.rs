// Ch17 — Strings & Text
//
// CONCEPTS:
//   String vs &str     — owned heap string vs borrowed slice
//   chars() iterator   — Unicode-aware character iteration
//   split / join       — break and reassemble strings
//   find / contains    — substring search
//   String formatting  — format!, repeat, padding

/// Convert a string to Title Case: each word's first letter is uppercased.
/// "hello world" → "Hello World"
pub fn to_title_case(s: &str) -> String {
    todo!("split on whitespace, capitalize first char of each word, rejoin with spaces")
}

/// Convert a string to snake_case.
/// "Hello World" → "hello_world"
/// "camelCaseWord" → "camel_case_word"
pub fn to_snake_case(s: &str) -> String {
    todo!("lowercase; replace spaces/hyphens with '_'; insert '_' before uppercase transitions")
}

/// Return the first `max_len` characters of `s`, or all of `s` if shorter.
/// Works on char boundaries, not byte boundaries.
pub fn truncate(s: &str, max_len: usize) -> &str {
    todo!("find the byte index of the max_len-th char boundary and slice")
}

/// Count the number of whitespace-delimited words in `s`.
pub fn word_count(s: &str) -> usize {
    todo!("s.split_whitespace().count()")
}

/// Return true if `haystack` contains ANY of the given `needles`.
pub fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    todo!("needles.iter().any(|n| haystack.contains(n))")
}

/// Pad `s` on the right with spaces until it reaches `width` characters.
/// Returns `s` unchanged if it is already >= `width`.
pub fn pad_right(s: &str, width: usize) -> String {
    todo!("use format!(\"{{:<width$}}\") or manual padding with repeat")
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── to_title_case ─────────────────────────────────────────

    #[test]
    #[ignore = "implement to_title_case"]
    fn test_title_case_basic() {
        assert_eq!(to_title_case("hello world"), "Hello World");
    }

    #[test]
    #[ignore = "implement to_title_case"]
    fn test_title_case_already_title() {
        assert_eq!(to_title_case("Hello World"), "Hello World");
    }

    #[test]
    #[ignore = "implement to_title_case"]
    fn test_title_case_single_word() {
        assert_eq!(to_title_case("rust"), "Rust");
    }

    #[test]
    #[ignore = "implement to_title_case"]
    fn test_title_case_empty() {
        assert_eq!(to_title_case(""), "");
    }

    // ── to_snake_case ─────────────────────────────────────────

    #[test]
    #[ignore = "implement to_snake_case"]
    fn test_snake_case_from_spaces() {
        assert_eq!(to_snake_case("Hello World"), "hello_world");
    }

    #[test]
    #[ignore = "implement to_snake_case"]
    fn test_snake_case_already_snake() {
        assert_eq!(to_snake_case("already_snake"), "already_snake");
    }

    #[test]
    #[ignore = "implement to_snake_case"]
    fn test_snake_case_empty() {
        assert_eq!(to_snake_case(""), "");
    }

    // ── truncate ──────────────────────────────────────────────

    #[test]
    #[ignore = "implement truncate"]
    fn test_truncate_shorter_than_max() {
        assert_eq!(truncate("hi", 10), "hi");
    }

    #[test]
    #[ignore = "implement truncate"]
    fn test_truncate_exact_length() {
        assert_eq!(truncate("hello", 5), "hello");
    }

    #[test]
    #[ignore = "implement truncate"]
    fn test_truncate_over_max() {
        assert_eq!(truncate("hello world", 5), "hello");
    }

    #[test]
    #[ignore = "implement truncate"]
    fn test_truncate_zero() {
        assert_eq!(truncate("hello", 0), "");
    }

    // ── word_count ────────────────────────────────────────────

    #[test]
    #[ignore = "implement word_count"]
    fn test_word_count_basic() {
        assert_eq!(word_count("one two three"), 3);
    }

    #[test]
    #[ignore = "implement word_count"]
    fn test_word_count_empty() {
        assert_eq!(word_count(""), 0);
    }

    #[test]
    #[ignore = "implement word_count"]
    fn test_word_count_extra_spaces() {
        assert_eq!(word_count("  one   two  "), 2);
    }

    // ── contains_any ─────────────────────────────────────────

    #[test]
    #[ignore = "implement contains_any"]
    fn test_contains_any_first_matches() {
        assert!(contains_any("hello world", &["hello", "xyz"]));
    }

    #[test]
    #[ignore = "implement contains_any"]
    fn test_contains_any_second_matches() {
        assert!(contains_any("hello world", &["xyz", "world"]));
    }

    #[test]
    #[ignore = "implement contains_any"]
    fn test_contains_any_none_match() {
        assert!(!contains_any("hello world", &["foo", "bar"]));
    }

    #[test]
    #[ignore = "implement contains_any"]
    fn test_contains_any_empty_needles() {
        assert!(!contains_any("hello", &[]));
    }

    // ── pad_right ─────────────────────────────────────────────

    #[test]
    #[ignore = "implement pad_right"]
    fn test_pad_right_pads() {
        assert_eq!(pad_right("hi", 5), "hi   ");
    }

    #[test]
    #[ignore = "implement pad_right"]
    fn test_pad_right_exact_width() {
        assert_eq!(pad_right("hello", 5), "hello");
    }

    #[test]
    #[ignore = "implement pad_right"]
    fn test_pad_right_already_wider() {
        assert_eq!(pad_right("toolong", 3), "toolong");
    }
}
