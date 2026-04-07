
#[derive(Debug, PartialEq)]
pub struct SearchResult<'a> {
    pub line_num: usize,
    pub line_text: &'a str,  // Borrowed reference (no copying!)
}
pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<SearchResult<'a>> {


    contents
        .lines()                                    // Split('\n') in C#
        .enumerate()                                // Select((line, i) => ...)
        .filter(|(_, line)| line.contains(pattern)) // Where(r => r.Contains(...))
        .map(|(i, line)| SearchResult {            // Select(... => new SearchResult)
            line_num: i + 1,
            line_text: line,
        })
        .collect()                                  // ToList()

}
pub fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<SearchResult<'a>> {
    let pattern_lower = pattern.to_lowercase();
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&pattern_lower))
        .map(|(i, line)| SearchResult {
            line_num: i + 1,
            line_text: line,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Test search with matches found
    #[test]
    fn test_search_with_matches() {
         let contents = "Rust is safe.\nRust is fast.\nC is old.";
         let results = search("Rust", contents);

         // 🎓 C# COMPARISON: Accessing struct fields
         // C#: Assert.Equal(2, results.Count);
         // Rust: assert_eq!(results.len(), 2);
         assert_eq!(results.len(), 2);

         // Access struct fields by name (not by position like tuples)
         // C#: Assert.Equal(1, results[0].LineNum);
         // Rust: assert_eq!(results[0].line_num, 1);
         assert_eq!(results[0].line_num, 1);
         assert_eq!(results[0].line_text, "Rust is safe.");

         assert_eq!(results[1].line_num, 2);
         assert_eq!(results[1].line_text, "Rust is fast.");
    }

    // TODO: Test search with no matches
    #[test]
    fn test_search_no_matches() {
          let contents = "Hello world";
          let results = search("Rust", contents);
          assert!(results.is_empty());
    }

    // TODO: Test case sensitivity
    #[test]
    fn test_case_sensitive() {
           let contents = "Rust\nrust\nRUST";
           let results = search("Rust", contents);
           assert_eq!(results.len(), 1);
    }

    // Test case insensitive search
    #[test]
    fn test_case_insensitive() {
          let contents = "Rust\nrust\nRUST";
          let results = search_case_insensitive("rust", contents);

          // Should match all 3 variations
          assert_eq!(results.len(), 3);

          // Verify all matches found
          assert_eq!(results[0].line_num, 1);
          assert_eq!(results[0].line_text, "Rust");

          assert_eq!(results[1].line_num, 2);
          assert_eq!(results[1].line_text, "rust");

          assert_eq!(results[2].line_num, 3);
          assert_eq!(results[2].line_text, "RUST");
    }
}
