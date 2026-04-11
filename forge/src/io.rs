// Ch18 — Input & Output
//
// CONCEPTS:
//   BufRead             — buffered reading; .lines() yields Result<String>
//   Write               — trait for writing bytes; write! / writeln! macros
//   io::Result<T>       — alias for Result<T, io::Error>
//   BufReader / Cursor  — wrap a Read to get BufRead; Cursor wraps &[u8]
//   ? operator          — propagate io::Error up the call stack

use std::io::{self, BufRead, Write};

/// Count the number of lines in `reader`.
/// An empty reader returns 0. A single line with no trailing newline returns 1.
pub fn count_lines(reader: impl BufRead) -> usize {
    todo!()
}

/// Read all whitespace-delimited words from `reader` into a Vec.
/// "hello  world\nrust" → ["hello", "world", "rust"]
pub fn read_words(reader: impl BufRead) -> Vec<String> {
    todo!()
}

/// Copy every byte from `reader` to `writer`, uppercasing ASCII letters.
/// Non-letter bytes are written unchanged.
pub fn copy_uppercased(reader: impl BufRead, writer: &mut impl Write) -> io::Result<()> {
    todo!()
}

/// Write each string in `lines` to `writer`, prefixed with its 1-based line number.
/// Line 1: "hello" is written as "1: hello\n"
pub fn write_numbered(lines: &[&str], writer: &mut impl Write) -> io::Result<()> {
    todo!()
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    // ── count_lines ───────────────────────────────────────────

    #[test]
    #[ignore = "implement count_lines"]
    fn test_count_lines_three_lines() {
        let input = Cursor::new("line1\nline2\nline3\n");
        assert_eq!(count_lines(input), 3);
    }

    #[test]
    #[ignore = "implement count_lines"]
    fn test_count_lines_empty() {
        assert_eq!(count_lines(Cursor::new("")), 0);
    }

    #[test]
    #[ignore = "implement count_lines"]
    fn test_count_lines_no_trailing_newline() {
        assert_eq!(count_lines(Cursor::new("hello")), 1);
    }

    // ── read_words ────────────────────────────────────────────

    #[test]
    #[ignore = "implement read_words"]
    fn test_read_words_basic() {
        let input = Cursor::new("hello world\nrust");
        assert_eq!(read_words(input), vec!["hello", "world", "rust"]);
    }

    #[test]
    #[ignore = "implement read_words"]
    fn test_read_words_extra_whitespace() {
        let input = Cursor::new("  one   two  ");
        assert_eq!(read_words(input), vec!["one", "two"]);
    }

    #[test]
    #[ignore = "implement read_words"]
    fn test_read_words_empty() {
        let result = read_words(Cursor::new(""));
        assert!(result.is_empty());
    }

    // ── copy_uppercased ───────────────────────────────────────

    #[test]
    #[ignore = "implement copy_uppercased"]
    fn test_copy_uppercased_letters() {
        let input = Cursor::new("hello");
        let mut output = Vec::new();
        copy_uppercased(input, &mut output).unwrap();
        assert_eq!(output, b"HELLO");
    }

    #[test]
    #[ignore = "implement copy_uppercased"]
    fn test_copy_uppercased_non_alpha_unchanged() {
        let input = Cursor::new("hi, 42!");
        let mut output = Vec::new();
        copy_uppercased(input, &mut output).unwrap();
        assert_eq!(output, b"HI, 42!");
    }

    // ── write_numbered ────────────────────────────────────────

    #[test]
    #[ignore = "implement write_numbered"]
    fn test_write_numbered_basic() {
        let mut output = Vec::new();
        write_numbered(&["alpha", "beta", "gamma"], &mut output).unwrap();
        let s = String::from_utf8(output).unwrap();
        assert_eq!(s, "1: alpha\n2: beta\n3: gamma\n");
    }

    #[test]
    #[ignore = "implement write_numbered"]
    fn test_write_numbered_empty() {
        let mut output = Vec::new();
        write_numbered(&[], &mut output).unwrap();
        assert!(output.is_empty());
    }
}
