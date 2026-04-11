// Capstone 1 — ripgrep Clone
//
// Builds on ch02-tour's minigrep but adds:
//   - Regex matching (not just string::contains)
//   - Recursive directory walking with .gitignore support
//   - Parallel file processing (rayon)
//   - Colored terminal output
//   - Line numbers and file paths in output

pub mod config;
pub mod matcher;
pub mod output;
pub mod search;
pub mod walker;

pub use config::Config;
pub use matcher::Match;

use anyhow::Result;

/// Entry point: run a search with the given config.
/// Returns the total number of matching lines found.
pub fn run(config: &Config) -> Result<u64> {
    todo!("walk paths in config, search each file in parallel, print matches, return count")
}
