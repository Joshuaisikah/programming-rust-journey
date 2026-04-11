// output.rs — Colored terminal output
//
// Formats match results for display:
//   path:line_num:highlighted_line
//
// Uses the `colored` crate. When --no-color is set, falls back to plain text.

use crate::matcher::Match;
use colored::Colorize;

/// Print a single match to stdout.
/// Highlights matched byte ranges in red; filename in purple; line number in green.
pub fn print_match(m: &Match, show_line_numbers: bool, color: bool) {
    todo!("format path, optional line_num, and highlighted line text")
}

/// Print a per-file count summary: "path: N matches"
pub fn print_count(path: &std::path::Path, count: usize, color: bool) {
    todo!("print path and count, colored if color=true")
}

/// Highlight the match ranges inside `line` in red.
/// Returns a String with ANSI codes inserted (or plain if `color=false`).
pub fn highlight_line(line: &str, ranges: &[std::ops::Range<usize>], color: bool) -> String {
    todo!("build string: plain text between ranges, colored text for ranges")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── highlight_line ────────────────────────────────────────

    #[test]
    #[ignore = "implement highlight_line"]
    fn test_highlight_no_color_returns_plain_line() {
        let s = highlight_line("hello world", &[6..11], false);
        assert_eq!(s, "hello world");
    }

    #[test]
    #[ignore = "implement highlight_line"]
    fn test_highlight_no_ranges_returns_original() {
        let s = highlight_line("nothing matched", &[], false);
        assert_eq!(s, "nothing matched");
    }

    #[test]
    #[ignore = "implement highlight_line"]
    fn test_highlight_with_color_contains_match_text() {
        let s = highlight_line("hello world", &[6..11], true);
        // The colored string should still contain "world" somewhere
        assert!(s.contains("world"));
    }

    #[test]
    #[ignore = "implement highlight_line"]
    fn test_highlight_full_line_match() {
        let s = highlight_line("abc", &[0..3], false);
        assert_eq!(s, "abc");
    }
}
