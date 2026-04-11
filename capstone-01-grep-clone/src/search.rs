// search.rs — Regex search within a single file
//
// This is the core search unit. Takes a compiled Regex and file contents,
// returns all matching lines with their match ranges.
//
// Builds directly on ch02-tour's SearchResult pattern but uses
// the `regex` crate instead of str::contains.

use std::path::PathBuf;
use regex::Regex;
use crate::matcher::Match;

/// Search `contents` for all lines matching `pattern`.
/// Returns one Match per matching line, with byte ranges for highlights.
pub fn search_file(
    path: PathBuf,
    contents: &str,
    pattern: &Regex,
) -> Vec<Match> {
    todo!("enumerate lines, find_iter to collect ranges, build Match for each hit")
}

/// Build a Regex from a pattern string.
/// If `ignore_case` is true, wrap pattern in (?i).
pub fn build_regex(pattern: &str, ignore_case: bool) -> Result<Regex, regex::Error> {
    todo!("prefix with (?i) when ignore_case, then Regex::new")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn re(s: &str) -> Regex { Regex::new(s).unwrap() }
    fn path() -> PathBuf { PathBuf::from("test.rs") }

    // ── search_file ───────────────────────────────────────────

    #[test]
    #[ignore = "implement search_file"]
    fn test_search_finds_matching_lines() {
        let contents = "fn foo() {}\nlet x = 1;\nfn bar() {}";
        let results = search_file(path(), contents, &re("fn"));
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].line_num, 1);
        assert_eq!(results[1].line_num, 3);
    }

    #[test]
    #[ignore = "implement search_file"]
    fn test_search_no_matches_returns_empty() {
        let contents = "hello world\nfoo bar";
        let results = search_file(path(), contents, &re("zzz"));
        assert!(results.is_empty());
    }

    #[test]
    #[ignore = "implement search_file"]
    fn test_search_records_match_ranges() {
        let contents = "hello world";
        let results = search_file(path(), contents, &re("world"));
        assert_eq!(results.len(), 1);
        // "world" starts at byte 6 in "hello world"
        assert_eq!(results[0].ranges[0], 6..11);
    }

    #[test]
    #[ignore = "implement search_file"]
    fn test_search_multiple_matches_on_same_line() {
        let contents = "aaa bbb aaa";
        let results = search_file(path(), contents, &re("aaa"));
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].ranges.len(), 2); // two occurrences on one line
    }

    #[test]
    #[ignore = "implement search_file"]
    fn test_search_preserves_path() {
        let p = PathBuf::from("src/main.rs");
        let results = search_file(p.clone(), "fn main() {}", &re("fn"));
        assert_eq!(results[0].path, p);
    }

    // ── build_regex ───────────────────────────────────────────

    #[test]
    #[ignore = "implement build_regex"]
    fn test_build_regex_case_sensitive() {
        let re = build_regex("Rust", false).unwrap();
        assert!(re.is_match("Rust is great"));
        assert!(!re.is_match("rust is great"));
    }

    #[test]
    #[ignore = "implement build_regex"]
    fn test_build_regex_case_insensitive() {
        let re = build_regex("rust", true).unwrap();
        assert!(re.is_match("Rust is great"));
        assert!(re.is_match("RUST IS GREAT"));
    }

    #[test]
    #[ignore = "implement build_regex"]
    fn test_build_regex_invalid_returns_error() {
        assert!(build_regex("[invalid", false).is_err());
    }
}
