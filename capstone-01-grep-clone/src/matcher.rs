// matcher.rs — Match result types
//
// Holds the data for one line that matched the search pattern.
// Keeps references into the source string to avoid copying.

use std::path::PathBuf;

/// One line that matched the search pattern.
#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    /// The file this match came from.
    pub path: PathBuf,
    /// 1-based line number.
    pub line_num: usize,
    /// The full text of the matching line.
    pub line: String,
    /// Byte ranges within `line` that matched the pattern.
    pub ranges: Vec<std::ops::Range<usize>>,
}

impl Match {
    /// Create a new Match.
    pub fn new(
        path: PathBuf,
        line_num: usize,
        line: String,
        ranges: Vec<std::ops::Range<usize>>,
    ) -> Self {
        todo!("construct Match")
    }

    /// Return true if there is at least one matching range.
    pub fn has_matches(&self) -> bool {
        todo!("!self.ranges.is_empty()")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "implement Match::new"]
    fn test_match_new_stores_fields() {
        let m = Match::new(
            PathBuf::from("src/main.rs"),
            42,
            "fn main() {}".to_string(),
            vec![0..2],
        );
        assert_eq!(m.path, PathBuf::from("src/main.rs"));
        assert_eq!(m.line_num, 42);
        assert_eq!(m.line, "fn main() {}");
        assert_eq!(m.ranges.len(), 1);
    }

    #[test]
    #[ignore = "implement Match::has_matches"]
    fn test_has_matches_true() {
        let m = Match::new(PathBuf::from("f.rs"), 1, "hello".into(), vec![0..3]);
        assert!(m.has_matches());
    }

    #[test]
    #[ignore = "implement Match::has_matches"]
    fn test_has_matches_false_when_no_ranges() {
        let m = Match::new(PathBuf::from("f.rs"), 1, "hello".into(), vec![]);
        assert!(!m.has_matches());
    }
}
