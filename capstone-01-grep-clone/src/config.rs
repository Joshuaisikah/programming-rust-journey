// config.rs — CLI argument parsing (clap-based)
//
// Extends ch02-tour's Config to support:
//   - Regex patterns (not just string literals)
//   - Directory or file targets
//   - Flags: case-insensitive, line numbers, count-only, color

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rgrep", about = "A fast ripgrep-like search tool")]
pub struct Config {
    /// The regex pattern to search for
    pub pattern: String,

    /// Files or directories to search (defaults to current dir)
    #[arg(default_value = ".")]
    pub paths: Vec<std::path::PathBuf>,

    /// Case-insensitive matching
    #[arg(short = 'i', long)]
    pub ignore_case: bool,

    /// Show line numbers in output
    #[arg(short = 'n', long)]
    pub line_numbers: bool,

    /// Only print a count of matching lines per file
    #[arg(short = 'c', long)]
    pub count_only: bool,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    /// Maximum number of results to show (0 = unlimited)
    #[arg(short = 'm', long, default_value = "0")]
    pub max_results: usize,

    /// Follow symbolic links
    #[arg(short = 'L', long)]
    pub follow_links: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let cfg = Config::try_parse_from(["rgrep", "hello"]).unwrap();
        assert_eq!(cfg.pattern, "hello");
        assert!(!cfg.ignore_case);
        assert!(!cfg.line_numbers);
        assert!(!cfg.count_only);
        assert!(!cfg.no_color);
        assert_eq!(cfg.max_results, 0);
    }

    #[test]
    fn test_config_short_flags() {
        let cfg = Config::try_parse_from(["rgrep", "-i", "-n", "TODO", "src/"]).unwrap();
        assert!(cfg.ignore_case);
        assert!(cfg.line_numbers);
        assert_eq!(cfg.pattern, "TODO");
    }

    #[test]
    fn test_config_count_flag() {
        let cfg = Config::try_parse_from(["rgrep", "-c", "error"]).unwrap();
        assert!(cfg.count_only);
    }

    #[test]
    fn test_config_max_results() {
        let cfg = Config::try_parse_from(["rgrep", "-m", "10", "foo"]).unwrap();
        assert_eq!(cfg.max_results, 10);
    }

    #[test]
    fn test_config_multiple_paths() {
        let cfg = Config::try_parse_from(["rgrep", "fn", "src/", "tests/"]).unwrap();
        assert_eq!(cfg.paths.len(), 2);
    }
}
