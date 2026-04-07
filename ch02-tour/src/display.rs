use crate::search::SearchResult;
pub fn display_results<'a>(filename: &str, matches: &[SearchResult<'a>]) {
    if matches.is_empty() {
        println!("No matches found");
        return;  // Early return, just like C#
    }
    println!("\n{}:", filename);
    for result in matches {
        println!("  {}: {}", result.line_num, result.line_text.trim());
    }
    println!("\nFound {} match(es)", matches.len());
}

pub fn display_error(error_msg: &str) {
    eprintln!("Error: {}", error_msg);
}
pub fn display_usage(program_name: &str) {
    eprintln!("Usage: {} <pattern> <filename>", program_name);
    eprintln!("Example: {} \"TODO\" src/main.rs", program_name);
}
